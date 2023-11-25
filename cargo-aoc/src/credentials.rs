use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub struct CredentialsManager {
    session_token: Option<String>,
}

impl CredentialsManager {
    /// Gets a reference to the local credentials.toml file
    fn get_credentials_file() -> PathBuf {
        let mut path_buf = ProjectDirs::from("com.github", "gobanos", "cargo-aoc")
            .map(|dirs| dirs.config_dir().to_path_buf())
            .expect("Home directory path could not be retrieved from the operating system");
        fs::create_dir_all(path_buf.as_path()).expect("Config directory could not be created");
        path_buf.push("credentials.toml");
        path_buf
    }

    /// Gets a reference to the old local credentials.toml file
    fn get_old_credentials_file() -> PathBuf {
        let mut path_buf = std::env::current_exe().expect("Could not find current path");
        path_buf.set_file_name("credentials.toml");
        path_buf
    }

    /// Gets an instance of the CredentialManager
    pub fn new() -> Self {
        // Gets a reference to the local credentials.toml file
        let path_buf = CredentialsManager::get_credentials_file();

        // If that doesn't exist yet, check the legacy path
        if !path_buf.exists() {
            let old_path_buf = CredentialsManager::get_old_credentials_file();
            if old_path_buf.exists() {
                // copy and delete, in case that they are on different file systems
                fs::copy(old_path_buf.as_path(), path_buf.as_path())
                    .expect("Couldn't copy credentials to new location");
                fs::remove_file(old_path_buf.as_path())
                    .expect("Couldn't delete old credentials file");
            }
        }

        // Reads it
        let token: Option<String> = match fs::read_to_string(path_buf) {
            // If we can read the TOML file
            Ok(content) => {
                // Parse the credentials.toml file
                let creds: toml::Value = content.parse().expect("Failed to parse credentials.toml");
                // Returns the parsed credentials' session value.
                // (or None, if we did not get anything.)
                creds["session"].as_str().map(|s| s.into())
            }
            // If we cant, just say that we did not get the token
            Err(_) => None,
        };

        CredentialsManager {
            session_token: token,
        }
    }

    /// Attemps to get the session token of the user if it is referenced
    /// in the credentials.toml file. Returns an error otherwise.
    pub fn get_session_token(&self) -> Result<String, String> {
        self.session_token
            .clone()
            .ok_or_else(|| "No session token available".into())
    }

    /// Attemps to set the session token of the user in credentials.toml
    /// Returns an error in case of an IO error or something ...
    pub fn set_session_token(&mut self, token: String) -> Result<(), std::io::Error> {
        // Gets a reference to the local credentials.toml file
        let toml_str = format!("session = \"{}\"", token);
        // Sets the information of this struct
        self.session_token = Some(token);
        let path_buf = CredentialsManager::get_credentials_file();

        // Writes the session token to the file
        fs::write(path_buf, toml_str)
    }
}
