// https://www.hackerrank.com/challenges/diagonal-difference/problem

pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut d1 = 0;
    let mut d2 = 0;
    for i in 0..n {
        d1 += arr[i][i];
        d2 += arr[i][n - 1 - i];
    }
    (d1 - d2).abs()
}

#[cfg(test)]
mod task11 {
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(diagonal_difference(&[
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12]
        ]), 15);
    }
}