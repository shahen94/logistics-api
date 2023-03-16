use super::tracking::client::TrackingApi;

pub struct DHL {
    pub tracking: TrackingApi,
}

impl DHL {
    pub fn new(api_key: &'static str) -> Self {
        return DHL {
            tracking: TrackingApi::new(api_key),
        };
    }
}
