use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn largest_overlap(img1: Vec<Vec<i32>>, img2: Vec<Vec<i32>>) -> i32 {
        let mut overlap = 0;
        let n = img1.len();
        for y_offset in 0..n {
            for x_offset in 0..n {
                let mut overlap1 = 0;
                let mut overlap2 = 0;
                let left_down = (x_offset, y_offset);
                let right_up = (n, n);
                for x in left_down.0..right_up.0 {
                    for y in left_down.1..right_up.1 {
                        if img1[x][y] == 1 && img2[x - x_offset][y - y_offset] == 1 {
                            overlap1 += 1;
                        }
                        if img2[x][y] == 1 && img1[x - x_offset][y - y_offset] == 1 {
                            overlap2 += 1;
                        }
                    }
                }
                overlap = std::cmp::max(overlap, std::cmp::max(overlap1, overlap2));

                let mut overlap1 = 0;
                let mut overlap2 = 0;
                let left_down = (x_offset, 0);
                let right_up = (n, n - y_offset);
                for x in left_down.0..right_up.0 {
                    for y in left_down.1..right_up.1 {
                        if img1[x][y] == 1 && img2[x - x_offset][y + y_offset] == 1 {
                            overlap1 += 1;
                        }
                        if img2[x][y] == 1 && img1[x - x_offset][y + y_offset] == 1 {
                            overlap2 += 1;
                        }
                    }
                }
                overlap = std::cmp::max(overlap, std::cmp::max(overlap1, overlap2));
            }
        }
        overlap
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        img1: Vec<Vec<i32>>,
        img2: Vec<Vec<i32>>,
        answer: i32,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        img1: vec![vec![1, 1, 0], vec![0, 1, 0], vec![0, 1, 0]],
        img2: vec![vec![0, 0, 0], vec![0, 1, 1], vec![0, 0, 1]],
        answer: 3,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        img1: vec![vec![1]],
        img2: vec![vec![1]],
        answer: 1,
    };
    test_cases.push(t2);

    let t3 = TestCase {
        img1: vec![vec![0]],
        img2: vec![vec![0]],
        answer: 0,
    };
    test_cases.push(t3);

    let t4 = TestCase {
        img1: vec![
            vec![0, 0, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ],
        img2: vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![1, 0, 0, 0, 0],
        ],
        answer: 1,
    };
    test_cases.push(t4);

    for tc in test_cases {
        assert_eq!(tc.answer, Solution::largest_overlap(tc.img1, tc.img2));
    }
}
