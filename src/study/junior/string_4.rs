use crate::Solution;
use std::ops::{BitAndAssign, BitOrAssign};

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        use std::ops::{BitAndAssign, BitOrAssign};
        let mut map = vec![0; 26];
        let start = 'a' as usize;
        let mut bitmap = 0u32;
        for c in s.chars() {
            let i = c as usize - start;
            map[i] += 1;
            if map[i] == 0 {
                bitmap.bitand_assign(((1 << 26) - 1) - (1 << i))
            } else {
                bitmap.bitor_assign(1 << i);
            }
        }
        for c in t.chars() {
            let i = c as usize - start;
            map[i] -= 1;
            if map[i] == 0 {
                bitmap.bitand_assign(((1 << 26) - 1) - (1 << i))
            } else {
                bitmap.bitor_assign(1 << i);
            }
        }
        bitmap == 0
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
