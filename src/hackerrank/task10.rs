// https://www.hackerrank.com/challenges/sock-merchant/problem

pub fn sock_merchant(ar: &[i32]) -> i32 {
    let mut counts = std::collections::HashMap::new();
    for &s in ar {
        *counts.entry(s).or_insert(0) += 1;
    }
    counts.values().map(|&c| c / 2).sum()
}

#[cfg(test)]
mod task10 {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(sock_merchant(&[10, 20, 20, 10, 10, 30, 50, 10, 20]), 3);
    }
}