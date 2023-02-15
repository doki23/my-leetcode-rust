use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut total = 0;
        for i in 0..n {
            let mut dist_map = std::collections::HashMap::with_capacity(n * (n - 1));
            for j in 0..n {
                if i != j {
                    let x_dis = (points[j][0] - points[i][0]) * (points[j][0] - points[i][0]);
                    let y_dis = (points[j][1] - points[i][1]) * (points[j][1] - points[i][1]);
                    let dist = x_dis + y_dis;
                    if let Some(cnt) = dist_map.get_mut(&dist) {
                        *cnt += 1;
                    } else {
                        dist_map.insert(dist, 1);
                    }
                }
            }
            for (_, cnt) in dist_map {
                total += cnt * (cnt - 1);
            }
        }
        total
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        points: Vec<Vec<i32>>,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    // let t1 = TestCase {
    //     points: vec![vec![0, 0], vec![1, 0], vec![2, 0]],
    //     answer: 2,
    // };
    // test_cases.push(t1);
    //
    // let t2 = TestCase {
    //     points: vec![vec![1, 1], vec![2, 2], vec![3, 3]],
    //     answer: 2,
    // };
    // test_cases.push(t2);
    //
    // let t3 = TestCase {
    //     points: vec![vec![1, 1]],
    //     answer: 0,
    // };
    // test_cases.push(t3);

    let t4 = TestCase {
        points: vec![vec![0, 0], vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]],
        answer: 20,
    };
    test_cases.push(t4);

    for tc in test_cases {
        assert_eq!(tc.answer, Solution::number_of_boomerangs(tc.points));
    }
}
