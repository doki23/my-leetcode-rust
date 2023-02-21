use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn first_uniq_char(s: String) -> i32 {
        let mut map = [-1i32; 26];
        let start = 'a' as u8;
        let mut offset = 0usize;
        for (i, b) in s.bytes().enumerate() {
            offset = (b - start) as usize;
            if map[offset] == -1 {
                map[offset] = i as i32;
            } else {
                map[offset] = -2;
            }
        }
        map.into_iter().filter(|v| *v >= 0).min().unwrap_or(-1)
    }
}

#[test]
fn test() {
    struct TestCase {
        s: String,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        s: "leetcode".to_string(),
        answer: 0,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        s: "loveleetcode".to_string(),
        answer: 2,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        s: "aabb".to_string(),
        answer: -1,
    };
    test_cases.push(t3);

    let t4 = TestCase {
        s: "a".to_string(),
        answer: 0,
    };
    test_cases.push(t4);

    for tc in test_cases {
        let ans = Solution::first_uniq_char(tc.s);
        assert_eq!(tc.answer, ans);
    }
}
