use mm_errors::{MeshError, Result};
use regorus::{Engine, Value};
pub fn allow(policy: &str, sensitivity: &str, separated: bool, risk: f64) -> Result<bool> {
    let mut e = Engine::new();
    e.add_policy("mandate.rego".into(), policy.into())
        .map_err(|x| MeshError::Dependency(format!("Regorus policy: {x}")))?;
    let input =
        serde_json::json!({"sensitivity":sensitivity,"separation_of_duty":separated,"risk":risk});
    e.set_input(
        Value::from_json_str(&input.to_string()).map_err(|x| MeshError::Invalid(x.to_string()))?,
    );
    let v = e
        .eval_rule("data.mandate_mesh.mandate.allow".into())
        .map_err(|x| MeshError::Dependency(format!("Regorus eval: {x}")))?;
    Ok(v == Value::Bool(true))
}
#[cfg(test)]
mod tests {
    use super::*;
    const P: &str = include_str!("../../../policies/mandate.rego");
    #[test]
    fn real_policy() {
        assert!(allow(P, "internal", true, 0.1).unwrap());
        assert!(!allow(P, "restricted", true, 0.1).unwrap())
    }
}
