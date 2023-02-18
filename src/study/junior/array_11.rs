use crate::Solution;

impl Solution {
    pub fn rotate_matrix(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();
        for i in 0..=(len - 1) / 2 {
            for j in i..len - i - 1 {
                let mut x = i;
                let mut y = j;
                let mut tmp = matrix[x][y];
                for _ in 0..4 {
                    let next_x = y;
                    let next_y = len - x - 1;
                    tmp = std::mem::replace(&mut matrix[next_x][next_y], tmp);
                    x = next_x;
                    y = next_y;
                }
            }
        }
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        matrix: Vec<Vec<i32>>,
        answer: Vec<Vec<i32>>,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        matrix: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        answer: vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]],
    };
    test_cases.push(t1);

    let t2 = TestCase {
        matrix: vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ],
        answer: vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ],
    };
    test_cases.push(t2);

    for mut tc in test_cases {
        Solution::rotate_matrix(&mut tc.matrix);
        assert_eq!(tc.answer, tc.matrix);
    }
}
