use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut first_zero = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                first_zero += 1;
            }
        }
        for i in 0..nums.len() {
            if nums[i] != 0 && first_zero < i {
                nums.swap(i, first_zero);
                first_zero += 1;
                while first_zero < nums.len() && nums[first_zero] != 0 {
                    first_zero += 1;
                }
            }
        }
    }
}
