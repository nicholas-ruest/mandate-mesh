use mm_errors::{MeshError, Result};
pub const POLICY: &str = include_str!("../../../policies/mandate.rego");
pub fn authorize(sensitivity: &str, separated: bool, risk: f64) -> Result<()> {
    if mm_regorus::allow(POLICY, sensitivity, separated, risk)? {
        Ok(())
    } else {
        Err(MeshError::Denied(
            "mandate policy rejected objective".into(),
        ))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn restricted_denied() {
        assert!(authorize("restricted", true, 0.1).is_err())
    }
}
