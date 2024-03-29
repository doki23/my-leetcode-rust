use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn reverse(x: i32) -> i32 {
        let mut v = vec![];
        let sign = if x < 0 { -1 } else { 1 };
        let mut x: u64 = (x * sign) as u64;
        let mut mu: u64 = 1;
        loop {
            if x == 0 {
                break;
            }
            let a: u64 = x % (mu * 10);
            x -= a;
            v.push(a / mu);
            mu *= 10;
        }
        let mut ans = 0;
        for i in 0..v.len() {
            ans += v[i] * 10u64.pow((v.len() - 1 - i) as u32);
            if ans > i32::MAX as u64 {
                return 0;
            }
        }
        sign * (ans as i32)
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        x: i32,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        x: 123,
        answer: 321,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        x: -123,
        answer: -321,
    };
    test_cases.push(t2);

    let t3 = TestCase { x: 120, answer: 21 };
    test_cases.push(t3);

    let t4 = TestCase { x: 0, answer: 0 };
    test_cases.push(t4);

    let t5 = TestCase {
        x: 1234567899,
        answer: 0,
    };
    test_cases.push(t5);

    for tc in test_cases {
        let ans = Solution::reverse(tc.x);
        assert_eq!(tc.answer, ans);
    }
}
