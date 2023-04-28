use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use rust_cms_json_parser::{
    in_network_file_dto::InNetworkFile,
    index_file_parsing::{
        self,
        csv_meta_repository::CsvMetaRepository,
        index_file::IndexFile,
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
    let example_index_file_path =
        "./price-transparency-guide/examples/table-of-contents/table-of-contents-sample.json";
    index_file_parsing::parse_index_file_from_path(example_index_file_path);
}

#[test]
fn it_writes_to_csv_meta_repo() {
    let repo: CsvMetaRepository = CsvMetaRepository {
        files_csv_path: "./db/files.csv",
        links_csv_path: "./db/links.csv",
        plans_csv_path: "./db/plans.csv",
    };

    let file_id: usize = repo.add_file(&mut FileRowInput {
        url: "example.com/file.json",
        filename: "file.json",
        reporting_entity_name: "drew",
        reporting_entity_type: "type1",
    });

    let plan_id: usize = repo.add_plan(&mut PlanInput {
        plan_name: "plan1",
        plan_id_type: "type1",
        plan_market_type: "market_type1",
        plan_id: "0000000",
    });

    repo.add_link(&mut DbLinkInput {
        from_id: file_id,
        from_type: "rate_file",
        to_id: plan_id,
        to_type: "plan",
    });
}

// #[test]
// fn it_sends_and_receives_deserialized_items_to_channel() {
//     let path = Path::new("price-transparency-guide")
//         .join("examples")
//         .join("table-of-contents")
//         .join("table-of-contents-sample.json");

//     let file = File::open(path).unwrap();
//     let index_file_deserializer = &mut serde_json::Deserializer::from_reader(file);

//     // deserialize by sending value on the channel
//     let (sender, receiver) = channel::<ReportingStructure>();
// let channel_visitor = ChannelVisitor { sender };
// index_file_deserializer
//     .deserialize_newtype_struct("Channel", channel_visitor)
//     .unwrap();

// // receive value
// let value = receiver.recv().unwrap();
// println!("Received value: {:?}", value);
// }

#[test]
fn it_deserializes_via_channels() {
    let path = Path::new("price-transparency-guide")
        .join("examples")
        .join("table-of-contents")
        .join("table-of-contents-sample.json");

    let file = File::open(path).unwrap();
    let index_file: IndexFile = serde_json::from_reader(file).unwrap();
    println!("got index file! {:?}", index_file.reporting_entity_name);
    for reporting_structure in index_file.reporting_structure {
        println!("{:?}", reporting_structure);
    }
}
