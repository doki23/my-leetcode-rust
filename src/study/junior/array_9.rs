use crate::Solution;

impl Solution {
    #[cfg(test)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ans = vec![0; 2];
        let mut map = std::collections::HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            map.insert(num, i as i32);
        }
        for (i, num) in nums.iter().enumerate() {
            let another = target - num;
            match map.get(&another) {
                Some(j) if i as i32 != *j => {
                    ans[0] = i as i32;
                    ans[1] = *j;
                }
                _ => {}
            }
        }
        ans
    }
}
