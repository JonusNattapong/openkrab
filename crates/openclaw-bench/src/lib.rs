// Benchmarks placeholder — users should run `cargo bench -p openclaw-bench`
pub fn heavy_work(n: usize) -> usize {
    (0..n).fold(0usize, |acc, i| acc.wrapping_add(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heavy_work() {
        assert_eq!(
            heavy_work(10),
            (0..10).fold(0usize, |acc, i| acc.wrapping_add(i))
        );
    }
}
