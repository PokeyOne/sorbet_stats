//! This module contains everything to do with the raw metrics file that Sorbet
//! provides.

#[cfg(test)]
mod tests;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RawMetrics {
    pub repo: String,
    pub sha: String,
    pub status: String,
    pub branch: String,
    pub timestamp: String,
    pub uuid: String,
    pub metrics: Vec<Metric>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Metric {
    pub name: String,
    pub value: Option<u32>
}

impl RawMetrics {
    pub fn new(raw: &str) -> Result<RawMetrics, serde_json::Error> {
        serde_json::from_str(raw)
    }
}