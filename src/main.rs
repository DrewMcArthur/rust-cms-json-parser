mod index_file_parsing;
use std::{env, sync::Arc, time::Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = _get_path_arg(&args);

    let start = Instant::now();

    {
        index_file_parsing::parse_index_file_async(path);
    }

    let elapsed = start.elapsed();
    println!("Time Elapsed: {:.2?}", elapsed);
}

fn _get_path_arg(args: &Vec<String>) -> Arc<String> {
    match args.len() {
        1 => Arc::new(
            "./price-transparency-guide/examples/table-of-contents/table-of-contents-sample.json"
                .to_string(),
        ),
        _ => Arc::new(args[1].to_string()),
    }
}
