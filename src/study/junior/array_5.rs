use crate::Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut single = 0;
        for num in nums {
            single ^= num;
        }
        single
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        nums: Vec<i32>,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        nums: vec![2, 2, 1],
        answer: 1,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        nums: vec![4, 1, 2, 1, 2],
        answer: 4,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        nums: vec![1],
        answer: 1,
    };
    test_cases.push(t3);

    for tc in test_cases {
        let answer = Solution::single_number(tc.nums);
        assert_eq!(tc.answer, answer);
    }
}
