use std::{path::{Path, PathBuf}, fs};

use rust_cms_json_parser::in_network_file_dto::InNetworkFile;

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
