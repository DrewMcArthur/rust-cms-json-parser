use std::{
    fs::File,
    io::{stdout, Write},
    sync::{mpsc::sync_channel, Arc},
    thread,
};

use serde::de::DeserializeSeed;

use crate::index_file_parsing::{
    index_file::{IndexFile, IndexFileMetadata},
    meta_repository_trait::DbLinkInput,
    seed_deserialization::IndexFileSeed,
};

use self::{
    csv_meta_repository::CsvMetaRepository,
    index_file::ReportingStructure,
    meta_repository_trait::{BatchedMetaRepository, FileRowInput, MetaRepository, PlanInput},
    mysql_meta_repository::MysqlMetaRepository,
};

pub mod csv_meta_repository;
pub mod index_file;
pub mod meta_repository_trait;
mod mysql_meta_repository;
mod seed_deserialization;

// given a path to a local index file,
// deserialize it and its reporting structures,
// and send files and plan info to DB.
#[allow(dead_code)]
pub fn parse_index_file_sync(path: &str) {
    // get reporting_entity_name & type, publish file & get id
    println!("reading from {path}");
    let file = File::open(path).unwrap();
    let index_file: IndexFile = serde_json::from_reader(file).unwrap();

    let repo = _get_repo();

    let index_file_id = repo
        .add_file(FileRowInput {
            url: path,
            filename: "index",
            reporting_entity_name: &index_file.reporting_entity_name,
            reporting_entity_type: &index_file.reporting_entity_type,
        })
        .unwrap()
        .to_owned();

    for node in index_file.reporting_structure {
        handle_reporting_structure(
            index_file_id,
            &index_file.reporting_entity_name,
            &index_file.reporting_entity_type,
            &node,
        );
    }
}

fn handle_reporting_structure(
    index_file_id: usize,
    reporting_entity_name: &str,
    reporting_entity_type: &str,
    node: &ReportingStructure,
) {
    let repo = _get_mysql_repo();
    let mut file_ids: Vec<usize> = vec![];

    let plan_ids = repo.add_plans(
        node.reporting_plans
            .iter()
            .map(|plan| PlanInput {
                plan_name: plan.plan_name.as_str(),
                plan_id: plan.plan_id.as_str(),
                plan_id_type: plan.plan_id_type.as_str(),
                plan_market_type: plan.plan_market_type.as_str(),
            })
            .collect(),
    );

    if node.in_network_files.is_some() {
        file_ids = repo.add_files(
            node.in_network_files
                .as_ref()
                .unwrap()
                .iter()
                .map(|file| FileRowInput {
                    url: &file.location,
                    filename: _get_filename_from_url(&file.location),
                    reporting_entity_name,
                    reporting_entity_type,
                })
                .collect(),
        );
    }

    if node.allowed_amount_file.is_some() {
        let aa_file = node.allowed_amount_file.as_ref().unwrap();
        let aa_file_id = repo.add_file(FileRowInput {
            url: &aa_file.location.as_str(),
            filename: &aa_file.description.as_str(),
            reporting_entity_name: reporting_entity_name,
            reporting_entity_type: reporting_entity_type,
        });
        if aa_file_id.is_some() {
            file_ids.push(aa_file_id.unwrap().clone());
        }
    }

    for file_id in &file_ids {
        repo.add_link(DbLinkInput {
            from_id: index_file_id.clone(),
            from_type: "index_file",
            to_id: file_id.clone(),
            to_type: "rate_file",
        });

        for plan_id in &plan_ids {
            repo.add_link(DbLinkInput {
                from_id: plan_id.clone(),
                from_type: "plan",
                to_id: file_id.clone(),
                to_type: "rate_file",
            });
        }
    }
}

fn _get_repo() -> CsvMetaRepository {
    csv_meta_repository::CsvMetaRepository {
        files_csv_path: "./db/files.csv",
        links_csv_path: "./db/links.csv",
        plans_csv_path: "./db/plans.csv",
    }
}
fn _get_mysql_repo() -> MysqlMetaRepository {
    MysqlMetaRepository::new()
}

fn _get_filename_from_url(url: &str) -> &str {
    return url.split("/").last().unwrap();
}

// next thing to try: add another sender for the metadata?
// then when we receive that (which should be first)
// we can add the index file, and be able to handle the reporting_structure nodes
pub fn parse_index_file_async(path: Arc<String>) {
    let repo = _get_mysql_repo();
    let (metadata_sender, metadata_receiver) = sync_channel::<IndexFileMetadata>(0);
    let (reporting_structure_sender, reporting_structure_receiver) =
        sync_channel::<ReportingStructure>(32);

    println!("reading from {path}");

    // Deserialize in a separate thread.
    let thread_path = path.clone();
    let deserialize_thread = thread::spawn(move || {
        let file = File::open(thread_path.as_ref()).unwrap();
        let mut deserializer = serde_json::de::Deserializer::from_reader(&file);
        IndexFileSeed {
            metadata_sender,
            reporting_structure_sender,
        }
        .deserialize(&mut deserializer)
        .unwrap()
    });

    let metadata = metadata_receiver.recv().unwrap();
    let index_file_id = repo.add_file(FileRowInput {
        url: &path.to_owned(),
        filename: "index",
        reporting_entity_name: &metadata.reporting_entity_name,
        reporting_entity_type: &metadata.reporting_entity_type,
    });

    // TODO: get actual id
    let index_file_id = index_file_id.unwrap_or(0);

    println!("handling reporting structures...");
    let mut num_reporting_structures: usize = 0;
    while let Ok(value) = reporting_structure_receiver.recv() {
        // Process the deserialized values here.
        handle_reporting_structure(
            index_file_id,
            &metadata.reporting_entity_name,
            &metadata.reporting_entity_type,
            &value,
        );
        num_reporting_structures += 1;
        if num_reporting_structures % 10 == 0 {
            print!(
                "\rhandled {} reporting structures.",
                num_reporting_structures
            );
            stdout().flush().unwrap();
        }
    }

    println!("done handling reporting structures!");

    // You can also access the `File` after deserializing is complete.
    dbg!(deserialize_thread.join().unwrap());
}
