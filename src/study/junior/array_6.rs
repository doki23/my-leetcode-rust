use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut bitmap = vec![0; 1001];
        for i in 0..nums1.len() {
            bitmap[nums1[i] as usize] += 1;
        }
        let mut ans = vec![];
        for num in nums2 {
            if bitmap[num as usize] > 0 {
                bitmap[num as usize] -= 1;
                ans.push(num);
            }
        }
        ans
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        answer: Vec<i32>,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        nums1: vec![1, 2, 2, 1],
        nums2: vec![2, 2],
        answer: vec![2, 2],
    };
    test_cases.push(t1);

    let t2 = TestCase {
        nums1: vec![4, 9, 5],
        nums2: vec![4, 9],
        answer: vec![4, 9],
    };
    test_cases.push(t2);

    let t3 = TestCase {
        nums1: vec![1],
        nums2: vec![1],
        answer: vec![1],
    };
    test_cases.push(t3);

    let t4 = TestCase {
        nums1: vec![1, 1, 1, 1],
        nums2: vec![1, 1, 1],
        answer: vec![1, 1, 1],
    };
    test_cases.push(t4);

    let t5 = TestCase {
        nums1: vec![1, 1000],
        nums2: vec![1000],
        answer: vec![1000],
    };
    test_cases.push(t5);

    for mut tc in test_cases {
        let mut answer = Solution::intersect(tc.nums1, tc.nums2);
        answer.sort();
        tc.answer.sort();
        assert_eq!(tc.answer, answer);
    }
}
