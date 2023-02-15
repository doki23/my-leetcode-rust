use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut next = 1;
        while next < prices.len() {
            let curr_price = prices[next - 1];
            let next_price = prices[next];
            if next_price > curr_price {
                profit += next_price - curr_price;
            }
            next += 1;
        }
        profit
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        input: Vec<i32>,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        input: vec![7, 1, 5, 3, 6, 4],
        answer: 7,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        input: vec![1, 2, 3, 4, 5],
        answer: 4,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        input: vec![1],
        answer: 0,
    };
    test_cases.push(t3);

    let t4 = TestCase {
        input: vec![7, 6, 4, 3, 1],
        answer: 0,
    };
    test_cases.push(t4);

    let t5 = TestCase {
        input: vec![1, 1, 1, 1],
        answer: 0,
    };
    test_cases.push(t5);

    for tc in test_cases {
        assert_eq!(tc.answer, Solution::max_profit(tc.input));
    }
}
