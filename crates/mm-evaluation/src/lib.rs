use mm_types::Mandate;
#[derive(Debug, Clone)]
pub struct Metrics {
    pub coverage: f64,
    pub risk: f64,
    pub team_size: usize,
}
pub fn measure(m: &Mandate) -> Metrics {
    Metrics {
        coverage: m.tasks.len() as f64 / m.tasks.len().max(1) as f64,
        risk: m.risk,
        team_size: m.agents.len(),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn metric_shape() {
        let m = Mandate::new(
            mm_types::Objective {
                text: "x".into(),
                sensitivity: "public".into(),
            },
            vec![],
            vec![],
            0.2,
            "h".into(),
            "c".into(),
        );
        assert_eq!(measure(&m).team_size, 0)
    }
}
