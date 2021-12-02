[workspace]

[package]
name = "aoc-autobench"
version = "0.3.0"
authors = ["Grégory Obanos <gregory.obanos@gmail.com>"]

[dependencies]
{CRATE_NAME} = { path = "../../.." }
aoc-runner = "0.3"

[dev-dependencies]
criterion = "0.2.5"

{PROFILE}

[[bench]]
name = "aoc_benchmark"
harness = false
