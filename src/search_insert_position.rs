/**
 * [35] Search Insert Position
 *
 * Given a sorted array of distinct integers and a target value,
 * return the index if the target is found.
 * If not, return the index where it would be if it were inserted in order.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 */
struct Solution;

impl Solution {
	pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
		use std::cmp::Ordering;

		let mut low = 0 as i32;
		let mut high = nums.len() as i32;

		while low < high {
			let mid = low + (high - low) / 2;

			match nums[mid as usize].cmp(&target) {
				Ordering::Less => low = mid + 1,
				Ordering::Greater => high = mid,
				Ordering::Equal => return mid,
			}
		}

		low
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![1, 3, 5, 6];
		let target = 5;
		assert_eq!(Solution::search_insert(nums, target), 2);

		let nums = vec![1, 3, 5, 6];
		let target = 2;
		assert_eq!(Solution::search_insert(nums, target), 1);

		let nums = vec![1, 3, 5, 6];
		let target = 7;
		assert_eq!(Solution::search_insert(nums, target), 4);

		let nums = vec![1, 3, 5, 6];
		let target = 0;
		assert_eq!(Solution::search_insert(nums, target), 0);

		let nums = vec![1];
		let target = 0;
		assert_eq!(Solution::search_insert(nums, target), 0);

		let nums = vec![1];
		let target = 2;
		assert_eq!(Solution::search_insert(nums, target), 1);

		let nums = vec![1, 3];
		let target = 2;
		assert_eq!(Solution::search_insert(nums, target), 1);
	}
}
