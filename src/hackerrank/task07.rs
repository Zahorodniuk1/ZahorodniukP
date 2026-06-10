// https://www.hackerrank.com/challenges/between-two-sets/problem

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    (1..=100).filter(|&x| {
        a.iter().all(|&ai| x % ai == 0) &&
            b.iter().all(|&bi| bi % x == 0)
    }).count() as i32
}

#[cfg(test)]
mod task07 {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(get_total_x(&[2, 4], &[16, 32, 96]), 3);
    }
}