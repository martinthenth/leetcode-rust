/**
 * [167] Two Sum II - Input array is sorted
 *
 * Given a 1-indexed array of integers numbers that is already sorted in non-decreasing order,
 * find two numbers such that they add up to a specific target number.
 * Let these two numbers be numbers[index1] and numbers[index2]
 * where 1 <= first < second <= numbers.length.
 *
 * Return the indices of the two numbers, index1 and index2,
 * as an integer array [index1, index2] of length 2.
 */
struct Solution;

impl Solution {
	pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
		use std::cmp::Ordering;

		let mut low = 0;
		let mut high = numbers.len() - 1;

		loop {
			match (numbers[low] + numbers[high]).cmp(&target) {
				Ordering::Equal => return vec![(low + 1) as i32, (high + 1) as i32],
				Ordering::Greater => high -= 1,
				Ordering::Less => low += 1,
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![2, 7, 11, 15];
		let target = 9;
		let output = vec![1, 2];
		assert_eq!(Solution::two_sum(nums, target), output);

		let nums = vec![2, 3, 4];
		let target = 6;
		let output = vec![1, 3];
		assert_eq!(Solution::two_sum(nums, target), output);

		let nums = vec![-1, 0];
		let target = -1;
		let output = vec![1, 2];
		assert_eq!(Solution::two_sum(nums, target), output);
	}
}
