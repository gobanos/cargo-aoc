use std::fs;
use std::path::PathBuf;

pub struct CredentialsManager {
    session_token: Option<String>,
}

impl CredentialsManager {
    /// Gets a reference to the local credentials.toml file
    fn get_credentials_file() -> PathBuf {
        let mut path_buf = std::env::current_exe().expect("Could not find current path");
        path_buf.set_file_name("credentials.toml");
        path_buf
    }

    /// Gets an instance of the CredentialManager
    pub fn new() -> Self {
        // Gets a reference to the local credentials.toml file
        let path_buf = CredentialsManager::get_credentials_file();

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
            .ok_or("No session token available".into())
    }

    /// Attemps to set the session token of the user in credentials.toml
    /// Returns an error in case of an IO error or something ...
    pub fn set_session_token(&mut self, token: String) -> Result<(), std::io::Error> {
        // Sets the information of this struct
        self.session_token = Some(token.clone());
        // Gets a reference to the local credentials.toml file
        let path_buf = CredentialsManager::get_credentials_file();

        let toml_str = format!("session = \"{}\"", token);
        // Writes the session token to the file
        fs::write(path_buf, toml_str)
    }
}
