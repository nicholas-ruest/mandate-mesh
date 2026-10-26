use mm_errors::{MeshError, Result};
pub fn allocate(tasks: usize, total: u32) -> Result<Vec<u32>> {
    if tasks == 0 || total < tasks as u32 {
        return Err(MeshError::Invalid("insufficient budget".into()));
    }
    let each = total / tasks as u32;
    Ok((0..tasks).map(|_| each).collect())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded() {
        assert_eq!(allocate(3, 90).unwrap(), vec![30, 30, 30])
    }
}
