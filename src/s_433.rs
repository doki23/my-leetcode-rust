use crate::Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let mut q = std::collections::VecDeque::with_capacity(bank.len() + 1);
        let mut visited = std::collections::HashSet::new();

        q.push_back(start.clone());
        visited.insert(start);

        let gene = ["A", "C", "G", "T"];
        let mut steps = 0;

        while !q.is_empty() {
            for _ in 0..q.len() {
                let curr = q.pop_front().unwrap();
                if curr.eq(&end) {
                    return steps;
                }
                for s in gene {
                    for i in 0..8 {
                        let mut next = curr.clone();
                        next.replace_range(i..i + 1, s);
                        if bank.contains(&next) && !visited.contains(&next) {
                            q.push_back(next.clone());
                            visited.insert(next);
                        }
                    }
                }
            }
            steps += 1;
        }
        -1
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        start: String,
        end: String,
        bank: Vec<String>,
        answer: i32,
    }

    let mut test_cases = Vec::new();
    let t1 = TestCase {
        start: "AACCGGTT".to_string(),
        end: "AACCGGTA".to_string(),
        bank: ["AACCGGTA"].into_iter().map(ToOwned::to_owned).collect(),
        answer: 1,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        start: "AACCGGTT".to_string(),
        end: "AAACGGTA".to_string(),
        bank: ["AACCGGTA", "AACCGCTA", "AAACGGTA"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        answer: 2,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        start: "AAAAACCC".to_string(),
        end: "AACCCCCC".to_string(),
        bank: ["AAAACCCC", "AAACCCCC", "AACCCCCC"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
        answer: 3,
    };
    test_cases.push(t3);

    for tc in test_cases {
        assert_eq!(tc.answer, Solution::min_mutation(tc.start, tc.end, tc.bank));
    }
}
