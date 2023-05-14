use std::{fs::File, sync::mpsc::sync_channel, thread};

use serde::de::DeserializeSeed;

use crate::index_file_parsing::{
    index_file::{AsyncIndexFile, IndexFile, IndexFileMetadata},
    meta_repository_trait::DbLinkInput,
    seed_deserialization::FileSeed,
};

use self::{
    csv_meta_repository::CsvMetaRepository,
    index_file::ReportingStructure,
    meta_repository_trait::{FileRowInput, MetaRepository, PlanInput},
};

pub mod csv_meta_repository;
pub mod index_file;
pub mod meta_repository_trait;
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

    let index_file_id = repo.add_file(&mut FileRowInput {
        url: path,
        filename: "index",
        reporting_entity_name: &index_file.reporting_entity_name,
        reporting_entity_type: &index_file.reporting_entity_type,
    });

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
    let repo = _get_repo();
    let mut plan_ids: Vec<usize> = vec![];
    let mut file_ids: Vec<usize> = vec![];

    for plan in node.reporting_plans.as_slice() {
        plan_ids.push(repo.add_plan(&mut PlanInput::from_reporting_plan(&plan)));
    }

    for rate_file in node.in_network_files.as_slice() {
        file_ids.push(repo.add_file(&mut FileRowInput {
            url: &rate_file.location,
            filename: _get_filename_from_url(&rate_file.location).as_str(),
            reporting_entity_name: reporting_entity_name,
            reporting_entity_type: reporting_entity_type,
        }));
    }

    file_ids.push(repo.add_file(&mut FileRowInput {
        url: &node.allowed_amount_file.location,
        filename: &node.allowed_amount_file.description,
        reporting_entity_name: reporting_entity_name,
        reporting_entity_type: reporting_entity_type,
    }));

    for file_id in &file_ids {
        repo.add_link(&mut DbLinkInput {
            from_id: index_file_id.clone(),
            from_type: "index_file",
            to_id: file_id.clone(),
            to_type: "rate_file",
        });

        for plan_id in &plan_ids {
            repo.add_link(&mut DbLinkInput {
                from_id: plan_id.clone(),
                from_type: "plan",
                to_id: file_id.clone(),
                to_type: "rate_file",
            });
        }
    }
}

fn _get_repo() -> CsvMetaRepository<'static> {
    csv_meta_repository::CsvMetaRepository {
        files_csv_path: "./db/files.csv",
        links_csv_path: "./db/links.csv",
        plans_csv_path: "./db/plans.csv",
    }
}

fn _get_filename_from_url(url: &str) -> String {
    return url.split("/").last().unwrap().to_string();
}

// SYNC
pub fn parse_index_file_async(path: &'static str) {
    let repo: CsvMetaRepository = _get_repo();
    let (sender, receiver) = sync_channel::<ReportingStructure>(0);

    println!("reading from {path}");
    let file = File::open(path).unwrap();
    let index_file_metadata: IndexFileMetadata = serde_json::from_reader(&file).unwrap();
    let index_file_id = repo.add_file(&mut FileRowInput {
        url: path,
        filename: "index",
        reporting_entity_name: &index_file_metadata.reporting_entity_name,
        reporting_entity_type: &index_file_metadata.reporting_entity_type,
    });

    // Deserialize in a separate thread.
    let deserialize_thread = thread::spawn(move || {
        let file = File::open(path).unwrap();
        let mut deserializer = serde_json::de::Deserializer::from_reader(&file);
        let deserialized: AsyncIndexFile =
            FileSeed { sender }.deserialize(&mut deserializer).unwrap();
        deserialized
    });

    while let Ok(value) = receiver.recv() {
        // Process the deserialized values here.
        dbg!(&value);
        handle_reporting_structure(
            index_file_id,
            &index_file_metadata.reporting_entity_name,
            &index_file_metadata.reporting_entity_type,
            &value,
        )
    }

    // You can also access the `File` after deserializing is complete.
    dbg!(deserialize_thread.join().unwrap());
}
