/**
 * [283] Move Zeroes
 *
 * Given an integer array nums, move all 0's to the end of it while
 * maintaining the relative order of the non-zero elements.
 *
 * Note that you must do this in-place without making a copy of the array.
 */
struct Solution;

impl Solution {
	pub fn move_zeroes(nums: &mut Vec<i32>) {
		let mut i = 0;
		let mut j = 1;

		while j < nums.len() as i32 {
			if nums[i as usize] == 0 {
				if nums[j as usize] == 0 {
					j += 1;
				} else {
          nums.swap(i as usize, j as usize);
				}
			} else {
				i += 1;
				j += 1;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut nums = vec![0, 1, 0, 3, 12];
		let result = vec![1, 3, 12, 0, 0];
		Solution::move_zeroes(&mut nums);
		assert_eq!(nums, result);

		let mut nums = vec![0];
		let result = vec![0];
		Solution::move_zeroes(&mut nums);
		assert_eq!(nums, result);
	}
}
