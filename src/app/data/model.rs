use std::fmt::{self, Display};
use super::version::Version;
use serde::{Serialize, Deserialize};

/// # `Model` represents a model from a router brand
/// 
/// `name` - name of the brand
/// `versions` - router versions, ex: V1, V2, A1, A2 etc.,
///              this is brand specifc
/// `openwrt_version` - `Version` is the currently supported OpenWrt release for this model
/// `device_page` - link to detail information on this model
///
/// ## Example
/// 
/// Brand - TP-Link
/// Model - WR841ND
/// Versions - V10, V11
/// OpenWrt Version - 19.07
/// Device Page - https://openwrt.org/toh/tp-link/wr841nd
#[derive(Serialize, Deserialize, Debug)]
pub struct Model {
    name: String,
    versions: Vec<String>,
    openwrt_version: Version,
    device_page: String
}

impl Model {
    /// Creates a new instance of `Model` 
    pub fn new(name: String, versions: Vec<String>, openwrt_version: Version, device_page: String) -> Self {
        Self { name, versions, openwrt_version, device_page }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn versions(&self) -> &Vec<String> {
        &self.versions
    }

    pub fn openwrt_version(&self) -> &Version {
        &self.openwrt_version
    }

    pub fn device_page(&self) -> &String {
        &self.device_page
    }
}

impl Display for Model {
    /// Tries to json serialize `self`, else fallback  to debug format
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self)
            .unwrap_or(format!("{:#?}", self)))
    }
}