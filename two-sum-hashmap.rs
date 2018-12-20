// https://leetcode-cn.com/problems/two-sum/

// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

// 你可以假设每种输入只会对应一个答案。但是，你不能重复利用这个数组中同样的元素。

// 示例:

// 给定 nums = [2, 7, 11, 15], target = 9

// 因为 nums[0] + nums[1] = 2 + 7 = 9
// 所以返回 [0, 1]


use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_hash = HashMap::new();
        for i in 0..nums.len() {
          nums_hash.insert(nums[i], i as i32);
        }
        for i in 0..nums.len() {
            let key :i32 = target - nums[i];
            if nums_hash.contains_key(&key) {
                let idx2 = *nums_hash.get(&key).unwrap() as i32;
                let idx1: i32 = i as i32;
                 if idx1 != idx2 {
                    return vec![idx1, idx2]
                }
            }
        }
        
        return vec![0,0]
    }
}