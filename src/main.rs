mod index_file_parsing;
mod sync_array_serde;

fn main() {
    let example_index_file_path =
        "./price-transparency-guide/examples/table-of-contents/table-of-contents-sample.json";
    let results = index_file_parsing::parse_index_file_from_path(example_index_file_path);
    println!("done parsing.  results: {:?}", results);
}
