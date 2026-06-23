// Created by zhang-mo-peng-mo-peng-mo-mo-peng at 2026/06/21 11:13
// leetgo: 1.4.17
// https://leetcode.cn/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // 创建一个 HashMap 用于存储 {数值: 下标}
        // key 是数值，value 是该数值在 nums 中的索引
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            // 计算目标补数
            let complement = target - num;

            // 检查补数是否已经在 map 中
            if let Some(&prev_index) = map.get(&complement) {
                // 如果找到，返回两个下标
                return vec![prev_index as i32, i as i32];
            }

            // 如果没找到，将当前数值和下标存入 map
            map.insert(num, i);
        }

        // 题目保证一定有解，所以这里理论上不会执行到
        vec![]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
