[package]
name = "aoc-autobench"
version = "0.1.0"
authors = ["Grégory Obanos <gregory.obanos@gmail.com>"]

[dependencies]
{CRATE_NAME} = { path = "../../.." }
aoc-runner = "0.1.0"

[dev-dependencies]
criterion = "0.2.5"

[[bench]]
name = "aoc_benchmark"
harness = false
