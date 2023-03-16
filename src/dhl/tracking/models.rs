use serde::{Serialize, Deserialize};


/// Tracking response from DHL
#[derive(Debug, Serialize, Deserialize)]
pub struct Tracking {
  pub shipments: Vec<Shipment>,
}

/// Single Shipment details
#[derive(Debug, Serialize, Deserialize)]
pub struct Shipment {
  pub id: String,
  pub service: String,
  pub origin: PointLocation,
  pub destination: PointLocation,
  pub status: ShipmentStatus,
  pub details: ShipmentDetails,
  pub events: Vec<ShipmentEvent>,
}

/// Point location
#[derive(Debug, Serialize, Deserialize)]
pub struct PointLocation {
  pub address: Address,
  #[serde(rename = "servicePoint")]
  pub service_point: Option<ServicePoint>,
}

/// Address
#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
  #[serde(rename = "addressLocality")]
  pub address_locality: String,
}

/// Service point
#[derive(Debug, Serialize, Deserialize)]
pub struct ServicePoint {
  pub url: String,
  pub label: String,
}

/// Status of a shipment
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentStatus {
  pub timestamp: String,
  pub location: PointLocation,
  pub description: String,
}

/// Shipment details, contains proof of delivery, number of pieces and piece ids
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentDetails {
  #[serde(rename = "proofOfDelivery")]
  pub proof_of_delivery: ProofOfDelivery,

  #[serde(rename = "proofOfDeliverySignedAvailable")]
  pub proof_of_delivery_signed_available: bool,

  #[serde(rename = "totalNumberOfPieces")]
  pub total_number_of_pieces: i32,

  #[serde(rename = "pieceIds")]
  pub piece_ids: Vec<String>
}

/// Proof of delivery
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfDelivery {
  #[serde(rename = "signatureUrl")]
  pub signature_url: String,

  #[serde(rename = "documentUrl")]
  pub document_url: String,
}

/// Single shipment event
#[derive(Debug, Serialize, Deserialize)]
pub struct ShipmentEvent {
  pub timestamp: String,
  pub location: PointLocation,
  pub description: String,
}