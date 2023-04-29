use std::{
    fs::File,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread,
};

use crate::index_file_parsing::{
    index_file::IndexFile, meta_repository_trait::DbLinkInput, results_dto::IndexFileParsingResults,
};

use self::meta_repository_trait::{FileRowInput, MetaRepository, PlanInput};

pub mod csv_meta_repository;
pub mod index_file;
pub mod meta_repository_trait;
mod results_dto;

// given a path to a local index file,
// deserialize it and its reporting structures,
// and send files and plan info to DB.
pub fn parse_index_file_from_path(path: &'static str) -> IndexFileParsingResults {
    // get reporting_entity_name & type, publish file & get id
    println!("reading from {path}");
    let (file_sender, file_receiver) = sync_channel::<IndexFile>(0);
    let results_receiver = start_index_file_consumer(path, file_receiver);
    deserialize_index_file(path, file_sender);
    results_receiver.recv().unwrap()
}

fn start_index_file_consumer(
    path: &'static str,
    index_file_receiver: Receiver<IndexFile>,
) -> Receiver<IndexFileParsingResults> {
    let (results_sender, results_receiver) = sync_channel::<IndexFileParsingResults>(0);
    thread::spawn(move || {
        let index_file = index_file_receiver.recv().unwrap();

        let mut num_reporting_structures: i32 = 0;
        let mut num_plans: i32 = 0;
        let mut num_rate_files: i32 = 0;

        let repo = csv_meta_repository::CsvMetaRepository {
            files_csv_path: "./db/files.csv",
            links_csv_path: "./db/links.csv",
            plans_csv_path: "./db/plans.csv",
        };

        let index_file_id = repo.add_file(&mut FileRowInput {
            url: path.clone(),
            filename: "index",
            reporting_entity_name: &index_file.reporting_entity_name,
            reporting_entity_type: &index_file.reporting_entity_type,
        });

        for node in index_file.reporting_structure {
            println!("handling reporting structure {num_reporting_structures}");
            let mut plan_ids: Vec<usize> = vec![];
            let mut file_ids: Vec<usize> = vec![];

            for plan in node.reporting_plans {
                plan_ids.push(repo.add_plan(&mut PlanInput::from_reporting_plan(&plan)));
                num_plans += 1;
            }

            for rate_file in node.in_network_files {
                file_ids.push(repo.add_file(&mut FileRowInput {
                    url: &rate_file.location,
                    filename: _get_filename_from_url(&rate_file.location).as_str(),
                    reporting_entity_name: &index_file.reporting_entity_name,
                    reporting_entity_type: &index_file.reporting_entity_type,
                }));
                num_rate_files += 1;
            }

            file_ids.push(repo.add_file(&mut FileRowInput {
                url: &node.allowed_amount_file.location,
                filename: &node.allowed_amount_file.description,
                reporting_entity_name: &index_file.reporting_entity_name,
                reporting_entity_type: &index_file.reporting_entity_type,
            }));
            num_rate_files += 1;

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
            num_reporting_structures += 1;
        }

        results_sender
            .send(IndexFileParsingResults {
                index_file_id,
                reporting_entity_name: index_file.reporting_entity_name,
                reporting_entity_type: index_file.reporting_entity_type,
                num_reporting_structures,
                num_plans,
                num_rate_files,
            })
            .unwrap();
    });

    results_receiver
}

fn _get_filename_from_url(url: &str) -> String {
    return url.split("/").last().unwrap().to_string();
}

fn deserialize_index_file(path: &'static str, file_sender: SyncSender<IndexFile>) {
    thread::spawn(move || {
        let file = File::open(path).unwrap();
        file_sender
            .send(serde_json::from_reader(file).unwrap())
            .unwrap();
    })
    .join()
    .unwrap();
}
