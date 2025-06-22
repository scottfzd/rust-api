use axum::Json;
use axum::extract::Path;
use serde::Serialize;

#[derive(Serialize)]
pub struct Incident {
  id: u32,
  user_id: u32,
  incident_no: String,
  category: String,
  description: String,
  urgency: String,
  opened_at: u32,
}

pub async fn read_by_id(Path(id): Path<u32>) -> Json<Incident> {
  let incident = Incident {
    id: id,
    user_id: 9,
    incident_no: "19A83B".to_string(),
    category: "Atelier".to_string(),
    description: "Matériel défectueux".to_string(),
    urgency: "IMPORTANT".to_string(),
    opened_at: 1750180963,

  };

  Json(incident)
}