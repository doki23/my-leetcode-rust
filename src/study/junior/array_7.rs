use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let mut num = 0;
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            num = digits[i] + carry;
            digits[i] = num % 10;
            carry = num / 10;
            if i == 0 && carry == 1 {
                digits.insert(0, 1);
            }
        }
        digits
    }
}
