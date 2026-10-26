use mm_embedding::embed;
use mm_errors::Result;
use mm_ruvector::MandateIndex;
use mm_types::Analogue;
pub fn recall(index: &MandateIndex, text: &str) -> Result<Vec<Analogue>> {
    index.search(&embed(text)?, 3)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty_index() {
        assert!(recall(&MandateIndex::new().unwrap(), "objective")
            .unwrap()
            .is_empty())
    }
}
