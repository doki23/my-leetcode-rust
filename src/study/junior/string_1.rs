use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse_string(s: &mut Vec<char>) {
        let tail = s.len() - 1;
        for i in 0..=tail / 2 {
            s.swap(i, tail - i);
        }
    }
}
