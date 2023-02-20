use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn count_collisions(directions: String) -> i32 {
        let mut chars = directions.chars().collect::<Vec<char>>();
        let mut count = 0;
        let mut i = 0;
        let len = directions.len();
        while i < len {
            let curr_dir = chars[i];
            let right_dir = if i < len - 1 {
                Some(chars[i + 1])
            } else {
                None
            };
            match (curr_dir, right_dir) {
                ('R', Some('R'))
                | ('L', Some('R'))
                | ('L', Some('L'))
                | ('L', Some('S'))
                | ('S', Some('R'))
                | ('S', Some('S'))
                | (_, None) => i += 1,
                ('R', Some('L')) => {
                    count += 2;
                    let _ = std::mem::replace(&mut chars[i], 'S');
                    let _ = std::mem::replace(&mut chars[i + 1], 'S');
                    if i >= 1 {
                        i -= 1;
                    } else {
                        i += 1;
                    }
                }
                ('R', Some('S')) => {
                    count += 1;
                    let _ = std::mem::replace(&mut chars[i], 'S');
                    if i >= 1 {
                        i -= 1;
                    } else {
                        i += 1;
                    }
                }
                ('S', Some('L')) => {
                    count += 1;
                    let _ = std::mem::replace(&mut chars[i + 1], 'S');
                    i += 1;
                }
                _ => unreachable!(),
            }
        }
        count
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
        input: "RLRSLL".into(),
        answer: 5,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        input: "LLRR".into(),
        answer: 0,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        input: "L".into(),
        answer: 0,
    };
    test_cases.push(t3);

    let t4 = TestCase {
        input: "R".into(),
        answer: 0,
    };
    test_cases.push(t4);

    let t5 = TestCase {
        input: "RRRLLL".into(),
        answer: 6,
    };
    test_cases.push(t5);

    for tc in test_cases {
        assert_eq!(tc.answer, Solution::count_collisions(tc.input));
    }
}
