mod index_file_parsing;

fn main() {
    let example_index_file_path: &'static str =
        "./price-transparency-guide/examples/table-of-contents/table-of-contents-sample.json";
    index_file_parsing::parse_index_file_async(example_index_file_path);
}
