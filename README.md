# cms json parser

this repo hosts code designed to parse an in-network-rate file, as defined by the [cms gov price-transparency-guide](https://github.com/CMSgov/price-transparency-guide).

we implement a custom serde deserializer, which only keeps the RateObjects that match the billing codes we're looking for.

todo: 
- get billing codes from env var? 
- make available to python as a library?
- OR just directly interface with sqs and make this its own task.
- add benchmarking
