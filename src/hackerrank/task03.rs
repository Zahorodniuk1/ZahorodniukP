// https://www.hackerrank.com/challenges/staircase/problem

pub fn staircase(n: i32) {
    for i in 1..=n {
        let spaces = " ".repeat((n - i) as usize);
        let hashes = "#".repeat(i as usize);
        println!("{}{}", spaces, hashes);
    }
}

#[cfg(test)]
mod task03 {
    use super::*;

    #[test]
    fn test0() {
        staircase(6);
    }
}