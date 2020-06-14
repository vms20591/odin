use std::fmt::{self, Display};
use serde::{Serialize, Deserialize};
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
}

impl Display for Manufacturer {
    /// Tries to json serialize `self`, else fallback  to debug format    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Brand: {}\nModels: {}", self.name, serde_json::to_string_pretty(&self.models)
            .unwrap_or(format!("{:#?}", self.models)))
    }
}