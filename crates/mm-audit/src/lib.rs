use mm_errors::{MeshError, Result};
use mm_rvf::RvfArchive;
use mm_types::Mandate;
use sha2::{Digest, Sha256};
use std::path::Path;
#[derive(serde::Serialize, serde::Deserialize)]
struct Envelope {
    mandate: Mandate,
    previous: String,
    hash: String,
}
pub fn append(path: &Path, seq: u64, mandate: &Mandate, previous: &str) -> Result<String> {
    let body = serde_json::to_vec(&(mandate, previous))
        .map_err(|e| MeshError::Persistence(e.to_string()))?;
    let hash = hex::encode(Sha256::digest(body));
    let e = Envelope {
        mandate: mandate.clone(),
        previous: previous.into(),
        hash: hash.clone(),
    };
    RvfArchive::append(
        path,
        seq,
        &serde_json::to_vec(&e).map_err(|x| MeshError::Persistence(x.to_string()))?,
    )?;
    Ok(hash)
}
pub fn load(path: &Path) -> Result<Vec<Mandate>> {
    RvfArchive::read_all(path)?
        .into_iter()
        .map(|(_, b)| {
            serde_json::from_slice::<Envelope>(&b)
                .map(|e| e.mandate)
                .map_err(|e| MeshError::Persistence(e.to_string()))
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let d = tempfile::tempdir().unwrap();
        assert!(load(&d.path().join("x")).unwrap().is_empty())
    }
}
