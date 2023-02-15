use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let size = nums.len() as i32;
        let mut d = 0;
        loop {
            let hi = start + d;
            if hi < size && nums[hi as usize] == target {
                break;
            }
            let lo = start - d;
            if lo >= 0 && nums[lo as usize] == target {
                break;
            }
            if hi >= size && lo < 0 {
                break;
            }
            d += 1;
        }
        d
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        start: i32,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        nums: vec![1, 2, 3, 4, 5],
        target: 5,
        start: 3,
        answer: 1,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        nums: vec![1],
        target: 1,
        start: 0,
        answer: 0,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        nums: vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        target: 1,
        start: 0,
        answer: 0,
    };
    test_cases.push(t3);

    for tc in test_cases {
        assert_eq!(
            tc.answer,
            Solution::get_min_distance(tc.nums, tc.target, tc.start)
        );
    }
}
