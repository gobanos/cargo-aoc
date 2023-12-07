[workspace]

[package]
name = "aoc-autobench"
version = "0.3.0"
authors = ["Grégory Obanos <gregory.obanos@gmail.com>"]

[dependencies]
{CRATE_NAME} = { path = "../../.." }

# For release
aoc-runner = "0.3"
# For dev
# aoc-runner = { path = "../../../../aoc-runner" }

[dev-dependencies]
criterion = "0.5.1"

{PROFILE}

[[bench]]
name = "aoc_benchmark"
harness = false
