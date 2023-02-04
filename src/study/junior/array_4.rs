use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if !set.contains(&num) {
                set.insert(num);
            } else {
                return true;
            }
        }
        false
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        nums: Vec<i32>,
        answer: bool,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        nums: vec![1, 2, 3, 1],
        answer: true,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        nums: vec![1, 2, 3, 4],
        answer: false,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        nums: vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
        answer: true,
    };
    test_cases.push(t3);

    for tc in test_cases {
        let answer = Solution::contains_duplicate(tc.nums);
        assert_eq!(tc.answer, answer);
    }
}
