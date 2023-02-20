# cms json parser

![rust build workflow](https://github.com/DrewMcArthur/rust-cms-json-parser/actions/workflows/rust.yml/badge.svg)


this repo hosts code designed to parse an in-network-rate file, as defined by the [cms gov price-transparency-guide](https://github.com/CMSgov/price-transparency-guide).

we implement a custom serde deserializer, which only keeps the RateObjects that match the billing codes we're looking for.

todo: 
- get billing codes from env var? 
- make available to python as a library?
- OR just directly interface with sqs and make this its own task.
- add benchmarking

## installation

1. [install rustup](https://www.rust-lang.org/tools/install) (requires admin privileges)
2. build with `cargo build`, test with `cargo test`

## architecture

the main DTO is in `src/in_network_file_dto.rs`, and closely resembles 
[the cms schema for in network rate files](https://github.com/CMSgov/price-transparency-guide/tree/master/schemas/in-network-rates), 
and is designed to be deserializable from JSON via the serde crate.

the custom, filtered deserialization lives in `src/filtered_in_network_file.rs`.  
this is where i put the `filter_nodes` function, which handles how 
we deserialize the `in_network: Vec<InNetworkRateObject>` top-level key.
it's based on the similar implementation in the [serde documentation here](https://serde.rs/stream-array.html)

`src/node_filters.rs` defines the filtering functionality, 
as used by the above deserializing function.

`src/sqs/` has some boilerplate for sending/receiving messages via AWS SQS queues.  
I haven't actually hooked any of that part up yet, as I'm thinking 
this might be better suited for a binary library called by our python code.

unit tests can be found within each of the `src/` files, while integration tests against 
the `examples/` folder in the `price-transparency-guide` submodule live in `tests/`

## how/why

some of these in-network json files have turned out to be 10s of GiBs,
with 10s of thousands of `InNetworkRateObjects`.  This obviously is 
not feasible to store in memory, and was extremely performance-intensive to execute with python code.

instead, after realizing we often only wanted to parse the Rate Objects 
associated with specific billing codes, we realized we only ever needed 
a handful of these objects deserialized, rather than the entire list.

so, the goal of this project is to provide a library that:
- accepts a stream of bytes from a JSON in-network rate file
- deserializes to rust DTOs, filtering `InNetworkRateObjects` to only keep those with a matching billing code
- reserializes to a significantly smaller JSON object to be picked up by the python.
