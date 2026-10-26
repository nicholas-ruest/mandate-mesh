use mm_errors::{MeshError, Result};
use mm_types::Agent;
pub fn utility(team: &[Agent], cost_weight: f64) -> Result<f64> {
    if team.is_empty() {
        return Err(MeshError::Invalid("empty team".into()));
    }
    Ok(
        team.iter().map(|a| a.score).sum::<f64>() / team.len() as f64
            - cost_weight * team.len() as f64,
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn penalizes_size() {
        let a = mm_agent::synthetic_registry();
        assert!(utility(&a, 0.01).unwrap() < 1.0)
    }
}
