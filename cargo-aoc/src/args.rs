use std::env::args_os;
use std::ffi::OsString;

/// Clap does not play well with `cargo aoc` syntax (instead of `cargo-aoc` that works out of the box)
/// This iterator ignore the "aoc" argument in second position to mitigate this issue.
pub fn args_without_aoc() -> impl Iterator<Item = OsString> {
    args_os()
        .enumerate()
        .filter(|(i, arg)| *i != 1 || arg != "aoc")
        .map(|(_, arg)| arg)
}
