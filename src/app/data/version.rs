use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};

/// # `Version` represents some kind of version & a possible link to a version page
/// 
/// ## Example
/// 
/// Version("19.07", "https://openwrt.org/releases/19.07.2")
#[derive(Serialize, Deserialize, Debug)]
pub struct Version (String, String);

impl Version {
    /// Creates a new instance of `Version`
    pub fn new(version: String, link: String) -> Self {
        Self(version, link)
    }

    #[allow(dead_code)]
    pub fn version(&self) -> &String {
        &self.0
    }

    pub fn link(&self) -> &String {
        &self.1
    }
}

impl Display for Version {
    /// Tries to json serialize `self`, else fallback  to debug format
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self)
            .unwrap_or(format!("{:#?}", self)))
    }
}