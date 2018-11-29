use aoc_runner_internal::DayParts;
use std::error;
use std::fs;
use std::process;

pub struct ProjectManager {
    pub name: String,
    pub slug: String,
}

impl ProjectManager {
    pub fn new() -> Result<ProjectManager, Box<error::Error>> {
        let cargo: toml::Value = fs::read_to_string("Cargo.toml")?.parse()?;

        let crate_name = cargo
            .get("package")
            .ok_or("no field package in Cargo.toml")?
            .get("name")
            .ok_or("no field package.name in Cargo.toml")?
            .as_str()
            .ok_or("invalid crate name")?
            .to_string();

        let crate_slug = crate_name.replace('-', "_");

        Ok(ProjectManager {
            name: crate_name,
            slug: crate_slug,
        })
    }

    pub fn build_project(&self) -> Result<DayParts, Box<error::Error>> {
        let args = vec!["build", "--release", "--color=always"];

        let status = process::Command::new("cargo").args(&args).spawn()?.wait()?;

        if !status.success() {
            Err(format!(
                "cargo build failed with code {}",
                status.code().unwrap_or(-1)
            ))?;
        }

        DayParts::load()
    }
}
