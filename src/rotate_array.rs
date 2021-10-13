/**
 * [189] Rotate Array
 *
 * Given an array, rotate the array to the right by k steps, where k is non-negative.
 */
struct Solution;

impl Solution {
	pub fn rotate(nums: &mut Vec<i32>, k: i32) {
		let k = k % nums.len() as i32;

		Solution::reverse(nums, 0, (nums.len() - 1) as i32);
		Solution::reverse(nums, 0, k - 1);
		Solution::reverse(nums, k, (nums.len() - 1) as i32);
	}

	fn reverse(nums: &mut Vec<i32>, mut start: i32, mut end: i32) {
		while start < end {
			let temp = nums[start as usize] as i32;

			nums[start as usize] = nums[end as usize];
			nums[end as usize] = temp;

			start += 1;
			end -= 1;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
		let k = 3;
		let result = vec![5, 6, 7, 1, 2, 3, 4];
		Solution::rotate(&mut nums, k);
		assert_eq!(nums, result);

		let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
		let k = 10;
		let result = vec![5, 6, 7, 1, 2, 3, 4];
		Solution::rotate(&mut nums, k);
		assert_eq!(nums, result);

		let mut nums = vec![-1, -100, 3, 99];
		let k = 2;
		let result = vec![3, 99, -1, -100];
		Solution::rotate(&mut nums, k);
		assert_eq!(nums, result);

		let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
		let k = 0;
		let result = vec![1, 2, 3, 4, 5, 6, 7];
		Solution::rotate(&mut nums, k);
		assert_eq!(nums, result);
	}
}
