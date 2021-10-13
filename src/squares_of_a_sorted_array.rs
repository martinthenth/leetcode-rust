#[allow(dead_code)]
/**
 * [977] Squares of a Sorted Array
 *
 * Given an integer array nums sorted in non-decreasing order,
 * return an array of the squares of each number sorted in non-decreasing order.
 */
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
	let mut list = vec![0; nums.len()];
	let mut left = 0;
	let mut right = (nums.len() - 1) as i32;
	let mut position = nums.len() - 1;

	while left <= right {
		let a = nums[left as usize];
		let b = nums[right as usize];

		if a.abs() < b.abs() {
			list[position] = b.pow(2);
			right -= 1;
		} else {
			list[position] = a.pow(2);
			left += 1;
		}

		position = if position != 0 { position - 1 } else { 0 }
	}

	list
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let nums = vec![-4, -1, 0, 3, 10];
		let output = vec![0, 1, 9, 16, 100];
		assert_eq!(sorted_squares(nums), output);

		let nums = vec![-7, -3, 2, 3, 11];
		let output = vec![4, 9, 9, 49, 121];
		assert_eq!(sorted_squares(nums), output);
	}
}
