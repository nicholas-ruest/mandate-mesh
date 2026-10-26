use mm_errors::{MeshError, Result};
use mm_repository::Repository;
use mm_ruvector::MandateIndex;
use mm_types::Mandate;
use std::{path::PathBuf, sync::Mutex};
pub struct Orchestrator {
    repo: Repository,
    index: Mutex<MandateIndex>,
}
impl Orchestrator {
    pub fn open(path: PathBuf) -> Result<Self> {
        let repo = Repository::open(path)?;
        let mut index = MandateIndex::new()?;
        for m in repo.list()? {
            index.insert(&m.id, mm_embedding::embed(&m.objective.text)?)?
        }
        Ok(Self {
            repo,
            index: Mutex::new(index),
        })
    }
    pub fn create(&self, text: &str, sensitivity: &str, correlation_id: String) -> Result<Mandate> {
        if let Some(existing) = self
            .repo
            .list()?
            .into_iter()
            .find(|m| m.correlation_id == correlation_id)
        {
            return Ok(existing);
        }
        let objective = mm_objective::validate(text, sensitivity)?;
        let tasks = mm_task_graph::decompose(&objective.text)?;
        if !mm_task_graph::acyclic(&tasks) {
            return Err(MeshError::Invalid("task cycle".into()));
        }
        let registry = mm_seed::deterministic_agents();
        let _coverage = mm_capability::coverage(&tasks, &registry)?;
        let team = mm_team::assemble(&tasks, &registry)?;
        let separated = mm_team::separated(&team);
        let risk =
            mm_calibration::conformal_risk(&team.iter().map(|a| a.score).collect::<Vec<_>>(), 0.1)?;
        if mm_calibration::should_abstain(risk, 0.25) {
            return Err(MeshError::Denied("calibration abstained".into()));
        }
        mm_policy::authorize(&objective.sensitivity, separated, risk)?;
        let _utility = mm_optimizer::utility(&team, 0.01)?;
        let _budgets = mm_budget::allocate(tasks.len(), 90)?;
        let contract_hash = mm_contract::digest(&objective, &tasks, &team)?;
        let _scopes = mm_contract::scopes(&tasks);
        let mut mandate = Mandate::new(objective, tasks, team, risk, contract_hash, correlation_id);
        let analogues = {
            let idx = self
                .index
                .lock()
                .map_err(|_| MeshError::Dependency("index lock".into()))?;
            mm_recall::recall(&idx, &mandate.objective.text)?
        };
        if !analogues.is_empty() {
            mandate.status = "issued-with-analogue".into()
        }
        self.repo.save(&mandate)?;
        self.index
            .lock()
            .map_err(|_| MeshError::Dependency("index lock".into()))?
            .insert(&mandate.id, mm_embedding::embed(&mandate.objective.text)?)?;
        Ok(mandate)
    }
    pub fn list(&self) -> Result<Vec<Mandate>> {
        self.repo.list()
    }
    pub fn get(&self, id: &str) -> Result<Mandate> {
        self.repo.get(id)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertical_slice() {
        let d = tempfile::tempdir().unwrap();
        let o = Orchestrator::open(d.path().join("x.rvf")).unwrap();
        let m = o
            .create(
                "Investigate service latency and verify remediation",
                "internal",
                "test-corr".into(),
            )
            .unwrap();
        assert_eq!(o.get(&m.id).unwrap().agents.len(), 3);
        assert!(o
            .create(
                "Investigate service latency and verify remediation",
                "restricted",
                "c".into()
            )
            .is_err())
    }
}
