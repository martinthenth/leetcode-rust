/**
 * [1] Two Sum
 *
 * Given an array of integers nums and an integer target,
 * return indices of the two numbers such that they add up to target.
 *
 * You may assume that each input would have exactly one solution,
 * and you may not use the same element twice.
 *
 * You can return the answer in any order.
 */
struct Solution;

impl Solution {
	pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
		use std::collections::HashMap;

		let mut hashmap: HashMap<i32, i32> = HashMap::new();
		let mut i = 0;

		while i < nums.len() {
			let num = nums[i];
			let diff = target - num;

			match hashmap.get(&diff) {
				Some(&index) => return vec![index, i as i32],
				_ => {
					hashmap.insert(num, i as i32);
					i += 1;
				}
			}
		}

		vec![0, 0]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2, 7, 11, 15];
		let target = 9;
		let output = vec![0, 1];
		assert_eq!(Solution::two_sum(nums, target), output);

		let nums = vec![3, 2, 4];
		let target = 6;
		let output = vec![1, 2];
		assert_eq!(Solution::two_sum(nums, target), output);

		let nums = vec![3, 3];
		let target = 6;
		let output = vec![0, 1];
		assert_eq!(Solution::two_sum(nums, target), output);
	}
}
