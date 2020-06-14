use std::error::Error;
use colored::*;
use crate::log::Log;

mod data;
mod loader;

use data::{Model, Manufacturer, Version};

const OPENWRT_ROOT_PAGE: &str = "https://openwrt.org";
const ALL_DEVICES_PAGE: &str = "https://openwrt.org/toh/start";

/// Lists all brand names
pub fn list_brands() -> Result<(), Box<dyn Error>> {
    let manufacturers = loader::load_manufacturers()?;

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
pub fn list_models_for(brand: &str) -> Result<(), Box<dyn Error>> {
    let manufacturers = loader::load_manufacturers()?;
    
    if let Some(manufacturers) = manufacturers {
        let mut found = false;

        for manufacturer in &manufacturers {
            if brand.to_lowercase() == manufacturer.name().to_lowercase() {
                found = true;

                println!("{}", manufacturer);
                
                break;
            }
        }

        if !found {
            Log::print_error(format!("Found 0 brand(s)!"));    
        }
    }
    else {
        Log::print_error(format!("Found 0 brand(s)!"));
    }

    Ok(())
}

/// Lists all models for all available brands
pub fn list_models_for_all() -> Result<(), Box<dyn Error>> {
    let manufacturers = loader::load_manufacturers()?;
    
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