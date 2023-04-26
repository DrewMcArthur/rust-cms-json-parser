use std::fs::File;

use crate::index_file_parsing::{index_file::IndexFile, meta_repository_trait::DbLinkInput};

use self::meta_repository_trait::{FileRowInput, MetaRepository, PlanInput};

pub mod csv_meta_repository;
pub mod index_file;
pub mod meta_repository_trait;

// given a path to a local index file,
// deserialize it and its reporting structures,
// and send files and plan info to DB.
pub fn parse_index_file(path: &str) {
    // get reporting_entity_name & type, publish file & get id
    println!("reading from {path}");
    let file = File::open(path).unwrap();
    let index_file: IndexFile = serde_json::from_reader(file).unwrap();

    let repo = csv_meta_repository::CsvMetaRepository {
        files_csv_path: "./db/files.csv",
        links_csv_path: "./db/links.csv",
        plans_csv_path: "./db/plans.csv",
    };

    let index_file_id = repo.add_file(&mut FileRowInput {
        url: path,
        filename: "index",
        reporting_entity_name: &index_file.reporting_entity_name,
        reporting_entity_type: &index_file.reporting_entity_type,
    });

    for node in index_file.reporting_structure {
        let mut plan_ids: Vec<usize> = vec![];
        let mut file_ids: Vec<usize> = vec![];

        for plan in node.reporting_plans {
            plan_ids.push(repo.add_plan(&mut PlanInput::from_reporting_plan(&plan)));
        }

        for rate_file in node.in_network_files {
            file_ids.push(repo.add_file(&mut FileRowInput {
                url: &rate_file.location,
                filename: _get_filename_from_url(&rate_file.location).as_str(),
                reporting_entity_name: &index_file.reporting_entity_name,
                reporting_entity_type: &index_file.reporting_entity_type,
            }));
        }

        file_ids.push(repo.add_file(&mut FileRowInput {
            url: &node.allowed_amount_file.location,
            filename: &node.allowed_amount_file.description,
            reporting_entity_name: &index_file.reporting_entity_name,
            reporting_entity_type: &index_file.reporting_entity_type,
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
}

fn _get_filename_from_url(url: &str) -> String {
    return url.split("/").last().unwrap().to_string();
}
