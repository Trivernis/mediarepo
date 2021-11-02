use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InfoResponse {
    pub name: String,
    pub version: String,
    pub(crate) api_version: (u32, u32, u32),
}

impl InfoResponse {
    /// Creates a new info response
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            api_version: get_api_version(),
        }
    }

    /// Returns the api version of the crate
    pub fn api_version(&self) -> (u32, u32, u32) {
        self.api_version
    }
}

/// Retrieves the api version of the crate version in numbers
pub fn get_api_version() -> (u32, u32, u32) {
    let mut major = env!("CARGO_PKG_VERSION_MAJOR").to_string();
    let mut minor = env!("CARGO_PKG_VERSION_MINOR").to_string();
    let mut patch = env!("CARGO_PKG_VERSION_PATCH").to_string();
    major.retain(char::is_numeric);
    minor.retain(char::is_numeric);
    patch.retain(char::is_numeric);
    let major = major
        .parse::<u32>()
        .expect("Failed to parse major crate version");
    let minor = minor
        .parse::<u32>()
        .expect("Failed to parse minor crate version");
    let patch = patch
        .parse::<u32>()
        .expect("Failed to parse patch crate version");

    (major, minor, patch)
}

/// Checks if the api the client consumes is compatible to the one the server provides
pub fn check_apis_compatible(
    client_version: (u32, u32, u32),
    server_version: (u32, u32, u32),
) -> bool {
    // the major version must be the same while the client minor version can be lower than the servers
    // so that the client has access to all its supported functionality
    client_version.0 == server_version.0 && client_version.1 <= server_version.1
}
