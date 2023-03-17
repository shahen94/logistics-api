use super::models::Tracking;
use super::operation::Result;
use crate::dhl::constants;

/// DHL Tracking API
pub struct TrackingApi {
    pub base_url: String,
    pub api_key: &'static str,
}

impl TrackingApi {
    /// Create a new instance of the DHL Tracking API
    pub fn new(api_key: &'static str) -> Self {
        TrackingApi {
            base_url: String::from(constants::TRACKING_BASE_URL),
            api_key,
        }
    }

    /// Synchronous version of get_tracking
    pub fn get_tracking_sync(&self, tracking_number: &str) -> Result<Tracking> {
        let tracking: Tracking = reqwest::blocking::Client::new()
            .get(&self.base_url)
            .query(&[("trackingNumber", tracking_number), ("service", "express")])
            .header("DHL-API-Key", self.api_key)
            .send()?
            .json()?;

        Ok(tracking)
    }

    /// Get tracking information for a given tracking number using Async I/O
    pub async fn get_tracking(&self, tracking_number: &str) -> Result<Tracking> {
        let tracking: Tracking = reqwest::Client::new()
            .get(&self.base_url)
            .query(&[("trackingNumber", tracking_number), ("service", "express")])
            .header("DHL-API-Key", self.api_key)
            .send()
            .await?
            .json()
            .await?;

        Ok(tracking)
    }
}
