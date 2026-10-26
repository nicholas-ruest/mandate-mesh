use mm_agent::synthetic_registry;
use mm_types::Agent;
pub fn deterministic_agents() -> Vec<Agent> {
    synthetic_registry()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stable() {
        assert_eq!(deterministic_agents()[0].id, "a-research")
    }
}
