use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut first_zero = 0;
        while first_zero < nums.len() && nums[first_zero] != 0 {
            first_zero += 1;
        }
        for i in 0..nums.len() {
            if nums[i] != 0 && first_zero < i {
                nums.swap(i, first_zero);
                first_zero += 1;
                while first_zero < nums.len() && nums[first_zero] != 0 {
                    first_zero += 1;
                }
            }
        }
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        nums: Vec<i32>,
        answer: Vec<i32>,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        nums: vec![0, 1, 0, 3, 12],
        answer: vec![1, 3, 12, 0, 0],
    };
    test_cases.push(t1);

    for mut tc in test_cases {
        Solution::move_zeroes(&mut tc.nums);
        assert_eq!(tc.answer, tc.nums);
    }
}
