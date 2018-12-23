// https://leetcode-cn.com/problems/n-repeated-element-in-size-2n-array

use std::collections::HashMap;


impl Solution {
    pub fn repeated_n_times(a: Vec<i32>) -> i32 {
        let mut result = HashMap::new();
        
        for x in a.iter() {
            let v: i32 = if result.contains_key(x) {
                result[x] + 1
            } else {
                1
            };
            result.insert(x, v);
        }
        
        let n: i32 = a.len() as i32 / 2;
        let mut return_n: i32 = 0;
        for (&k, &v) in result.iter() {
            if v == n {
                return_n = *k;
            }  
        }
        return_n
    }
}
