use crate::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut p = 0;
        let mut q = s.len() - 1;
        let b = s.as_bytes();
        while p < q {
            let s1 = b[p];
            let s2 = b[q];
            if !Self::is_numeric(s1) && !Self::is_alphabet(s1) {
                p += 1;
                continue;
            }
            if !Self::is_numeric(s2) && !Self::is_alphabet(s2) {
                q -= 1;
                continue;
            }
            if s1 == s2
                || Self::is_alphabet(s1)
                    && Self::is_alphabet(s2)
                    && std::cmp::max(s2, s1) - std::cmp::min(s2, s1) == 32
            {
                p += 1;
                q -= 1;
                continue;
            } else {
                return false;
            }
        }
        true
    }

    #[inline]
    fn is_numeric(b: u8) -> bool {
        b >= 48 && b <= 57
    }

    #[inline]
    fn is_alphabet(b: u8) -> bool {
        b >= 97 && b <= 122 || b >= 65 && b <= 90
    }
}

#[test]
fn test_cases() {
    assert_eq!(false, Solution::is_palindrome("0P".to_string()));
    assert_eq!(
        true,
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    assert_eq!(false, Solution::is_palindrome("race a car".to_string()));
    assert_eq!(true, Solution::is_palindrome(" ".to_string()));
    assert_eq!(true, Solution::is_palindrome("  ".to_string()));
    assert_eq!(
        true,
        Solution::is_palindrome("aaaaaaaaaaaaaaaaaaaaaaaa".to_string())
    );
    assert_eq!(
        false,
        Solution::is_palindrome("aaaaaaaaaaaaaaaaaaaaaaaab".to_string())
    );
}
