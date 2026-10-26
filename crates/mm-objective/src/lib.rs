use mm_errors::{MeshError, Result};
use mm_types::Objective;
pub fn validate(text: &str, sensitivity: &str) -> Result<Objective> {
    let t = text.trim();
    if t.len() < 12 {
        return Err(MeshError::Invalid(
            "objective must be at least 12 characters".into(),
        ));
    }
    if !["public", "internal", "restricted"].contains(&sensitivity) {
        return Err(MeshError::Invalid("unknown sensitivity".into()));
    }
    Ok(Objective {
        text: t.into(),
        sensitivity: sensitivity.into(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_tiny() {
        assert!(validate("tiny", "public").is_err())
    }
}
