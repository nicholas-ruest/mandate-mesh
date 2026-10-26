use mm_agent::by_role;
use mm_errors::Result;
use mm_types::{Agent, Task};
pub fn assemble(tasks: &[Task], registry: &[Agent]) -> Result<Vec<Agent>> {
    let mut out = Vec::new();
    for t in tasks {
        let a = by_role(registry, &t.role)?.clone();
        if !out.iter().any(|x: &Agent| x.id == a.id) {
            out.push(a)
        }
    }
    Ok(out)
}
pub fn separated(team: &[Agent]) -> bool {
    let reviewers: Vec<_> = team
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == "reviewer"))
        .collect();
    let operators: Vec<_> = team
        .iter()
        .filter(|a| a.roles.iter().any(|r| r == "operator"))
        .collect();
    reviewers
        .iter()
        .all(|r| operators.iter().all(|o| r.id != o.id))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duties_separate() {
        assert!(separated(&mm_agent::synthetic_registry()))
    }
}
