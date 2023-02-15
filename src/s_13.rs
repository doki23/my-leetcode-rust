use crate::Solution;

// 字符          数值
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
//
impl Solution {
    #[cfg(test)]
    pub fn roman_to_int(s: String) -> i32 {
        let mapping: std::collections::HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let mut sum = 0;
        let mut last_ch = None;
        for roman_ch in s.chars() {
            sum += mapping[&roman_ch];
            if let Some(lch) = last_ch {
                if lch == 'I' && (roman_ch == 'V' || roman_ch == 'X') {
                    sum -= 2;
                } else if lch == 'X' && (roman_ch == 'L' || roman_ch == 'C') {
                    sum -= 20;
                } else if lch == 'C' && (roman_ch == 'D' || roman_ch == 'M') {
                    sum -= 200;
                }
            }
            if roman_ch == 'I' || roman_ch == 'X' || roman_ch == 'C' {
                last_ch.replace(roman_ch);
            } else {
                last_ch = None;
            }
        }
        sum
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        input: String,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        input: "IX".into(),
        answer: 9,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        input: "LVIII".into(),
        answer: 58,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        input: "MCMXCIV".into(),
        answer: 1994,
    };
    test_cases.push(t3);

    let t4 = TestCase {
        input: "I".into(),
        answer: 1,
    };
    test_cases.push(t4);

    for tc in test_cases {
        assert_eq!(tc.answer, Solution::roman_to_int(tc.input));
    }
}
