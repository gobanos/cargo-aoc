[workspace]

[package]
name = "aoc-autobuild"
version = "0.3.0"
authors = ["Grégory Obanos <gregory.obanos@gmail.com>"]
edition = "2021"

[dependencies]
{CRATE_NAME} = { path = "../../.." }

# For release
aoc-runner = "0.3"
# For dev
# aoc-runner = { path = "../../../../aoc-runner" }

{PROFILE}