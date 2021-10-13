/**
 * [704] Binary Search
 *
 * Given an array of integers nums which is sorted in ascending order,
 * and an integer target, write a function to search target in nums.
 * If target exists, then return its index. Otherwise, return -1.
 * You must write an algorithm with O(log n) runtime complexity.
 */
struct Solution;

impl Solution {
	pub fn search(nums: Vec<i32>, target: i32) -> i32 {
		use std::cmp::Ordering;

		let mut left = 0 as i32;
		let mut right = nums.len() as i32;

		while left < right {
			let mid = (left + right) / 2;

			match target.cmp(&nums[mid as usize]) {
				Ordering::Less => right = mid,
				Ordering::Greater => left = mid + 1,
				Ordering::Equal => return mid,
			}
		}

		-1
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![-1, 0, 3, 5, 9, 12];
		let target = 9;
		assert_eq!(Solution::search(nums, target), 4);

		let nums = vec![-1, 0, 3, 5, 9, 12];
		let target = 2;
		assert_eq!(Solution::search(nums, target), -1);

		let nums = vec![-1, 0, 3, 5, 9, 12];
		let target = 0;
		assert_eq!(Solution::search(nums, target), 1);

		let nums = vec![-1, 0, 3, 5, 9, 12];
		let target = 12;
		assert_eq!(Solution::search(nums, target), 5);

		let nums = vec![5];
		let target = 5;
		assert_eq!(Solution::search(nums, target), 0);

		let nums = vec![];
		let target = 0;
		assert_eq!(Solution::search(nums, target), -1);
	}
}
