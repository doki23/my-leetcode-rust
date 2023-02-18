use crate::Solution;

impl Solution {
    #[allow(dead_code, unused_assignments)]
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::ops::{BitAnd, BitOrAssign};
        let mut row: [u16; 9] = [0; 9];
        let mut column: [u16; 9] = [0; 9];
        let mut square: [u16; 9] = [0; 9];
        let mut shift: u16 = 0;
        for y in 0..9 {
            for x in 0..9 {
                let ch = board[x][y];
                if ch != '.' {
                    shift = 1 << (ch.to_digit(10).unwrap() as u8);
                    if row[x].bitand(shift) != 0
                        || column[y].bitand(shift) != 0
                        || square[x / 3 + (y / 3) * 3].bitand(shift) != 0
                    {
                        return false;
                    } else {
                        row[x].bitor_assign(shift);
                        column[y].bitor_assign(shift);
                        square[x / 3 + (y / 3) * 3].bitor_assign(shift);
                    }
                }
            }
        }
        true
    }
}

#[test]
fn test_cases() {
    struct TestCase {
        board: Vec<Vec<char>>,
        answer: bool,
    }

    let mut test_cases = Vec::new();

    let t1 = TestCase {
        board: vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        answer: true,
    };
    test_cases.push(t1);

    let t2 = TestCase {
        board: vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ],
        answer: false,
    };
    test_cases.push(t2);

    for tc in test_cases {
        let answer = Solution::is_valid_sudoku(tc.board);
        assert_eq!(tc.answer, answer);
    }
}
