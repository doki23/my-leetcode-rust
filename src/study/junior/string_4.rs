use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut map = vec![0; 26];
        let start = 'a' as usize;
        for c in s.chars() {
            let i = c as usize - start;
            map[i] += 1;
        }
        for c in t.chars() {
            let i = c as usize - start;
            if map[i] == 0 {
                return false;
            }
            map[i] -= 1;
        }
        true
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
    assert_eq!(
        Solution::is_anagram("ab".to_string(), "a".to_string()),
        false
    );
}
