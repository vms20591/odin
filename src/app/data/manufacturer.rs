use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};
use colored::*;
use super::model::Model;

/// # `Manufacturer` represents a router brand
/// 
/// `name` - name of the brand
/// `models` - list of `Model` available for this brand
/// 
/// ## Example
/// 
/// Brand - TP-Link
#[derive(Serialize, Deserialize, Debug)]
pub struct Manufacturer {
    name: String,
    models: Vec<Model>
}

impl Manufacturer {
    /// Creates a new instance of `Manufacturer`
    pub fn new(name: String, models: Vec<Model>) -> Self {
        Self { name, models }
    }

    /// Returns the number of models available
    pub fn count(&self) -> usize {
        self.models.len()
    }

    /// Returns manufacturer name
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn models(&self) -> &Vec<Model> {
        &self.models
    }

    fn display_as_table(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut console_width = 100;

        let index_width_percent = 5;
        let model_width_percent = 15;
        let version_width_percent = 15;
        let openwrt_version_width_percent = 35;
        let device_page_width_percent = 30;

        let mut index_width = 5;
        let mut model_width = 15;
        let mut version_width = 15;
        let mut openwrt_version_width = 35;
        let mut device_page_width = 30;

        if let Some((width, _)) = term_size::dimensions() {
            console_width = width;

            index_width = index_width_percent * width / 100;
            model_width = model_width_percent * width / 100;
            version_width = version_width_percent * width / 100;
            openwrt_version_width = openwrt_version_width_percent * width / 100;
            device_page_width = device_page_width_percent * width / 100;
        }

        write!(f, "{} {}", "Brand:".bold().bright_cyan(), self.name())?;
        write!(f, "\n")?;
        write!(f, "{}", format!("Found {} model(s)!", self.count()).bright_green())?;
        write!(f, "\n\n")?;

        write!(f, "{0:<1$}{2:3$}{4:5$}{6:7$}{8:9$}",
            "", index_width,
            "Model".bold().bright_cyan(), model_width,
            "Version".bold().bright_cyan(), version_width,
            "OpenWrt Version".bold().bright_cyan(), openwrt_version_width,
            "Device Page".bold().bright_cyan(), device_page_width    
        )?;
        write!(f, "\n")?;
        write!(f, "{0:<1$}{2:3$}{4:5$}{6:7$}{8:9$}",
            "", index_width,
            "-----".bold().bright_cyan(), model_width,
            "-------".bold().bright_cyan(), version_width,
            "---------------".bold().bright_cyan(), openwrt_version_width,
            "-----------".bold().bright_cyan(), device_page_width    
        )?;
        write!(f, "\n\n")?;

        let mut i = 1;

        for model in self.models() {
            let name = model.name();
            let version = if model.versions().len() > 0 {
                model.versions()
                    .join(", ")
                    .to_string()
            } else {
                "N/A".to_string()
            };
            let openwrt_version = if model.openwrt_version().link().len() > 0 {
                model.openwrt_version().link()
            } else {
                "N/A"
            };
            let device_page = if model.device_page().len() > 0 {
                model.device_page()
            } else {
                "N/A"
            };

            write!(f, "{0:<1$}{2:3$}{4:5$}{6:7$}{8:9$}",
                format!("{}.", i), index_width,
                name, model_width,
                version, version_width,
                openwrt_version, openwrt_version_width,
                device_page, device_page_width    
            )?;
            write!(f, "\n")?;

            i += 1;
        }
        
        write!(f, "\n")?;
        write!(f, "{}", format!("Found {} model(s)!", self.count()).bright_green())?;
        write!(f, "\n\n")?;
        write!(f, "{0:-^1$}", "", console_width)?;
        write!(f, "\n")
    }
}

impl Display for Manufacturer {
    /// Tries to display a table
    /// if it fails, tries to json serialize `self`
    /// else, fallback to debug format    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.display_as_table(f) {
            Ok(val) => Ok(val),
            Err(_) => {
                write!(f, "Brand: {}\nModels: {}", self.name, serde_json::to_string_pretty(&self.models)
                    .unwrap_or(format!("{:#?}", self.models)))
            }
        }
    }
}