//! Problem 1, two sum.
//!
//! Prob 1. Two Sum
//! ===============
//!
//! Given an array of integers `nums` and an integer `target`, return
//! indices of the two numbers such that they add up to `target`.
//!
//! You may assume that each input would have **exactly one solution**,
//! and you may not use the same element twice.
//!
//! You can return the answer in any order.
//!
//! **Example 1:**
//!
//! ```text
//! Input: nums = [2,7,11,15], target = 9
//! Output: [0,1]
//! Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//! ```
//!
//! **Example 2:**
//!
//! ```text
//! Output: [1,2]
//! ```
//!
//! **Example 3:**
//!
//! ```text
//! Input: nums = [3,3], target = 6
//! Output: [0,1]
//! ```
//!
//! **Constraints:**
//! - 2 ≤ `nums.len()` ≤ 10⁴
//! - -10⁹ ≤ `nums[i]` ≤ 10⁹
//! - 10⁹ ≤ target ≤ 10⁹`
//! - **Only one valid answer exists.**
//!
//! **Follow-up:** Can you come up with an algorithm that is less than
//! O(n²) time complexity?

/// Brute force solution with O(n²) time complexity
pub mod solution1 {
    pub struct Solution;

    // -- start --
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            for i in 0..nums.len() {
                // Every input has a solution, so `nums[i + 1]` will never be
                // out of bounds, the results will be found before then.
                for j in i + 1 .. nums.len() {
                    if nums[i] + nums[j] == target {
                        return vec![i as i32, j as i32];
                    }
                }
            }
            vec![]
        }
    }
    // -- end --

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example1() {
            assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        }

        #[test]
        fn example2() {
            assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        }

        #[test]
        fn example3() {
            assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        }
    }
}

/// Solution using hash table in O(n) time complexity
pub mod solution2 {
    pub struct Solution;

    // -- start --
    use std::collections::HashMap;

    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut table = HashMap::new();
            for (i, num) in nums.iter().enumerate() {
                if let Some(&j) = table.get(&(target - num)) {
                    return vec![j as i32, i as i32];
                } else {
                    table.insert(*num, i);
                }
            }
            vec![]
        }
    }
    // -- end --

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn example1() {
            assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        }

        #[test]
        fn example2() {
            assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        }

        #[test]
        fn example3() {
            assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        }
    }
}
