use mm_errors::Result;
use mm_types::{Agent, Objective, Task};
use sha2::{Digest, Sha256};
pub fn digest(objective: &Objective, tasks: &[Task], team: &[Agent]) -> Result<String> {
    let bytes = serde_json::to_vec(&(objective, tasks, team))
        .map_err(|e| mm_errors::MeshError::Invalid(e.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}
pub fn scopes(tasks: &[Task]) -> Vec<String> {
    tasks
        .iter()
        .map(|t| format!("mandate:{}:execute", t.id))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        let o = Objective {
            text: "x".into(),
            sensitivity: "public".into(),
        };
        assert_eq!(digest(&o, &[], &[]).unwrap(), digest(&o, &[], &[]).unwrap())
    }
}
