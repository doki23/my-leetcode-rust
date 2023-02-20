use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut tail = 0;
        let mut next = tail + 1;
        let len = nums.len();
        while next < len {
            let tail_val = nums[tail];
            let next_val = nums[next];

            if tail_val != next_val {
                tail += 1;
                nums[tail] = next_val;
                next += 1;
            } else {
                next = std::cmp::max(nums[next..].partition_point(|x| *x <= next_val), next + 1);
            }
        }
        (tail + 1) as i32
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        input: Vec<i32>,
        answer0: i32,
        answer1: Vec<i32>,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        input: vec![1, 1, 2],
        answer0: 2,
        answer1: vec![1, 2],
    };
    test_cases.push(t1);

    let t2 = TestCase {
        input: vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4],
        answer0: 5,
        answer1: vec![0, 1, 2, 3, 4],
    };
    test_cases.push(t2);

    let t3 = TestCase {
        input: vec![1],
        answer0: 1,
        answer1: vec![1],
    };
    test_cases.push(t3);

    let t4 = TestCase {
        input: vec![1, 2],
        answer0: 2,
        answer1: vec![1, 2],
    };
    test_cases.push(t4);

    for mut tc in test_cases {
        assert_eq!(tc.answer0, Solution::remove_duplicates(&mut tc.input));
        assert_eq!(tc.answer1, tc.input[..tc.answer0 as usize]);
    }
}
