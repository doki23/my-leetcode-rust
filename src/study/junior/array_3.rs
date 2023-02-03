use crate::Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        let mut swap = 0;
        let mut start = 0;
        loop {
            let mut i = start;
            let mut tmp = nums[start];
            let mut step = 0;
            while step == 0 || (step * k) % len != 0 {
                i = (i + k) % len;
                tmp = std::mem::replace(&mut nums[i], tmp);
                step += 1;
                swap += 1;
                if swap == len {
                    return;
                }
            }
            start += 1;
        }
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        nums: Vec<i32>,
        k: i32,
        answer: Vec<i32>,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        nums: vec![1, 2, 3, 4, 5, 6, 7],
        k: 3,
        answer: vec![5, 6, 7, 1, 2, 3, 4],
    };
    test_cases.push(t1);

    let t2 = TestCase {
        nums: vec![-1, -100, 3, 99],
        k: 2,
        answer: vec![3, 99, -1, -100],
    };
    test_cases.push(t2);

    let t3 = TestCase {
        nums: vec![1],
        k: 1,
        answer: vec![1],
    };
    test_cases.push(t3);

    let t4 = TestCase {
        nums: vec![1, 2, 3],
        k: 5,
        answer: vec![2, 3, 1],
    };
    test_cases.push(t4);

    let t5 = TestCase {
        nums: vec![1, 2, 3],
        k: 6,
        answer: vec![1, 2, 3],
    };
    test_cases.push(t5);

    for mut tc in test_cases {
        Solution::rotate(&mut tc.nums, tc.k);
        assert_eq!(tc.answer, tc.nums);
    }
}
