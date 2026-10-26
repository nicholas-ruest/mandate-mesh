use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    pub text: String,
    pub sensitivity: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Task {
    pub id: String,
    pub role: String,
    pub depends_on: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub roles: Vec<String>,
    pub score: f64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Mandate {
    pub id: String,
    pub objective: Objective,
    pub tasks: Vec<Task>,
    pub agents: Vec<Agent>,
    pub risk: f64,
    pub contract_hash: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub correlation_id: String,
}
impl Mandate {
    pub fn new(
        objective: Objective,
        tasks: Vec<Task>,
        agents: Vec<Agent>,
        risk: f64,
        contract_hash: String,
        correlation_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            objective,
            tasks,
            agents,
            risk,
            contract_hash,
            created_at: Utc::now(),
            status: "issued".into(),
            correlation_id,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateMandate {
    pub objective: String,
    pub sensitivity: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub code: String,
    pub message: String,
    pub correlation_id: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Health {
    pub status: String,
    pub persistence: String,
    pub policy: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Analogue {
    pub mandate_id: String,
    pub distance: f64,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mandate_has_identity() {
        let m = Mandate::new(
            Objective {
                text: "x".into(),
                sensitivity: "internal".into(),
            },
            vec![],
            vec![],
            0.1,
            "h".into(),
            "c".into(),
        );
        assert!(!m.id.is_empty())
    }
}
