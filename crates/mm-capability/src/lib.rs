use mm_agent::by_role;
use mm_errors::Result;
use mm_types::{Agent, Task};
pub fn coverage(tasks: &[Task], agents: &[Agent]) -> Result<f64> {
    for t in tasks {
        by_role(agents, &t.role)?;
    }
    Ok(tasks.len() as f64 / tasks.len().max(1) as f64)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full() {
        let a = mm_agent::synthetic_registry();
        let t = vec![Task {
            id: "x".into(),
            role: "reviewer".into(),
            depends_on: vec![],
        }];
        assert_eq!(coverage(&t, &a).unwrap(), 1.0)
    }
}
