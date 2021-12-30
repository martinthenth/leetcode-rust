/**
 * 1920. Build Array from Permutation
 *
 * Given a zero-based permutation nums (0-indexed), build an array ans of the same length
 * where ans[i] = nums[nums[i]] for each 0 <= i < nums.length and return it.
 *
 * A zero-based permutation nums is an array of distinct integers from 0 to nums.length - 1 (inclusive).
 */
struct Solution;

impl Solution {
	pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
		let n = nums.len();
		let mut i = 0;
		let mut result = vec![0; n];

		while i < n {
			result[i] = nums[nums[i] as usize];

			i += 1;
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let input = vec![0, 2, 1, 5, 3, 4];
		let output = vec![0, 1, 2, 4, 5, 3];
		assert_eq!(Solution::build_array(input), output);

		let input = vec![5, 0, 1, 2, 3, 4];
		let output = vec![4, 5, 0, 1, 2, 3];
		assert_eq!(Solution::build_array(input), output);
	}
}
