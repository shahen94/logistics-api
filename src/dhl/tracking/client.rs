use std::error::Error;

use reqwest::blocking::Client;

use crate::dhl::constants;

use super::models::Tracking;

/// DHL Tracking API
pub struct TrackingApi {
    pub base_url: String,
    pub api_key: &'static str,
    http_client: Client,
}

impl TrackingApi {
    /// Create a new instance of the DHL Tracking API
    pub fn new(api_key: &'static str) -> Self {
        TrackingApi {
            base_url: String::from(constants::TRACKING_BASE_URL),
            api_key,
            http_client: Client::new(),
        }
    }

    /// Get tracking information for a given tracking number
    pub fn get_tracking(&mut self, tracking_number: &str) -> Result<Tracking, Box<dyn Error>> {
        let res = self
            .http_client
            .get(&self.base_url)
            .query(&[("trackingNumber", tracking_number), ("service", "express")])
            .header("DHL-API-Key", self.api_key.clone())
            .send()?;

        let tracking: Tracking = res.json()?;

        Ok(tracking)
    }
}
