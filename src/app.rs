use std::error::Error;
use std::fs;
use colored::*;
use shellexpand;
use crate::log::Log;

mod data;
mod loader;

use data::{Model, Manufacturer, Version};

const OPENWRT_ROOT_PAGE: &str = "https://openwrt.org";
const ALL_DEVICES_PAGE: &str = "https://openwrt.org/toh/start";
const ODIN_HOME: &str = "~/.config/odin";
const ODIN_DEVICE_PAGE: &str = "~/.config/odin/devices.html";

pub struct App<'f> {
    file: Option<&'f str>
}

impl<'f> App<'f> {
    pub fn init(&self) {
        let _ = fs::create_dir_all(shellexpand::tilde(ODIN_HOME).trim());
    }

    pub fn new(file: Option<&'f str>) -> Self {
        let application = Self { file };
        
        application.init();

        application
    }

    /// Lists all brand names
    pub fn list_brands(&self) -> Result<(), Box<dyn Error>> {
        let manufacturers = loader::load_manufacturers(self.file)?;
    
        if let Some(manufacturers) = manufacturers {
            let mut i = 1;
    
            Log::print_ok(format!("Found {} brand(s)!", manufacturers.len()));
            println!();
    
            for brand in &manufacturers {
                println!("{}. {} - {} model(s)", i, brand.name().bold(), brand.count());
                
                i += 1;
            }
            
            println!();
            Log::print_ok(format!("Found {} brand(s)!", manufacturers.len()));
        }
        else {
            Log::print_error(format!("Found 0 brand(s)!"));
        }
    
        Ok(())
    }
    
    /// Lists all models for a given `brand`
    pub fn list_models_for(&self, brand: &str) -> Result<(), Box<dyn Error>> {
        let manufacturers = loader::load_manufacturers(self.file)?;
        
        if let Some(manufacturers) = manufacturers {
            let manufacturer = manufacturers.iter()
                .find(|manufacturer| brand.to_lowercase() == manufacturer.name().to_lowercase());

            if let Some(manufacturer) = manufacturer {
                println!("{}", manufacturer);
            }
            else {
                Log::print_error(format!("Found 0 brand(s)!"));
            }
        }
        else {
            Log::print_error(format!("Found 0 brand(s)!"));
        }
    
        Ok(())
    }
    
    /// Lists all models for all available brands
    pub fn list_models_for_all(&self) -> Result<(), Box<dyn Error>> {
        let manufacturers = loader::load_manufacturers(self.file)?;
        
        if let Some(manufacturers) = manufacturers {
            for manufacturer in &manufacturers {
                println!("{}", manufacturer);
            }
        }
        else {
            Log::print_error(format!("Found 0 brand(s)!"));
        }
    
        Ok(())
    }
}