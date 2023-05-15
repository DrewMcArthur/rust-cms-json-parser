use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use rust_cms_json_parser::{
    in_network_file_dto::InNetworkFile,
    index_file_parsing::{
        self,
        csv_meta_repository::CsvMetaRepository,
        meta_repository_trait::{DbLinkInput, FileRowInput, MetaRepository, PlanInput},
    },
};

fn file_name_is_json(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => ext.eq("json"),
        None => false,
    }
}

#[test]
fn it_deserializes_cms_examples() {
    let examples_dir = Path::new("price-transparency-guide")
        .join("examples")
        .join("in-network-rates");

    let paths = fs::read_dir(&examples_dir).expect("examples dir to exist");

    for path in paths {
        let path = path.expect("an existing path").path();
        if file_name_is_json(&path) {
            println!("testing with filename {}", path.to_string_lossy());
            let file_bytes = fs::read(path).expect("bytes from files");
            let file_obj: InNetworkFile = serde_json::from_slice(file_bytes.as_slice()).unwrap();
            assert!(
                file_obj.reporting_entity_name == "cms".to_string()
                    || file_obj.reporting_entity_name == "medicare".to_string()
            );
        }
    }
}

#[test]
fn it_parses_the_example_index_file() {
    let example_index_file_path = Arc::new(
        "./price-transparency-guide/examples/table-of-contents/table-of-contents-sample.json"
            .to_string(),
    );
    index_file_parsing::parse_index_file_async(example_index_file_path);
}

#[test]
fn it_writes_to_csv_meta_repo() {
    let repo: CsvMetaRepository = CsvMetaRepository {
        files_csv_path: "./db/files.csv",
        links_csv_path: "./db/links.csv",
        plans_csv_path: "./db/plans.csv",
    };

    let file_id: usize = repo.add_file(FileRowInput {
        url: "example.com/file.json",
        filename: "file.json",
        reporting_entity_name: "drew",
        reporting_entity_type: "type1",
    });

    let plan_id: usize = repo.add_plan(PlanInput {
        plan_name: "plan1",
        plan_id_type: "type1",
        plan_market_type: "market_type1",
        plan_id: "0000000",
    });

    repo.add_link(DbLinkInput {
        from_id: file_id,
        from_type: "rate_file",
        to_id: plan_id,
        to_type: "plan",
    });
}
