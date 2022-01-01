/**
 * 15. 3Sum
 *
 * Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]]
 * such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
 *
 * Notice that the solution set must not contain duplicate triplets.
 */
struct Solution;

impl Solution {
	pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
		use std::collections::{HashMap, HashSet};

		if nums.len() < 3 {
			return vec![];
		}

		let mut sorted_nums = nums.clone();
		sorted_nums.sort();

		let mut hash_map = HashMap::new();
		let mut hash_set = HashSet::new();
		let mut x = 0;
		let mut y;

		// Seed hash map
		for (index, num) in sorted_nums.iter().enumerate() {
			hash_map.insert(num, index);
		}

		//
		while x < nums.len() - 2 {
			y = x + 1;

			//
			while y < nums.len() - 1 {
				let missing_num = -1 * (sorted_nums[x] + sorted_nums[y]);

				match hash_map.get(&missing_num) {
					Some(&index) => {
						if index != x && index != y {
							//
							let mut vector = vec![sorted_nums[x], sorted_nums[y], missing_num];

							vector.sort();
							hash_set.insert(vector);
						}
					}
					None => {}
				}

				y += 1;
			}

			x += 1;
		}

		hash_set.into_iter().collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let input = vec![-1, 0, 1, 2, -1, -4];
		let output = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
		assert_eq!(Solution::three_sum(input), output);

		let input = vec![];
		let output: Vec<Vec<i32>> = vec![];
		assert_eq!(Solution::three_sum(input), output);

		let input = vec![0];
		let output: Vec<Vec<i32>> = vec![];
		assert_eq!(Solution::three_sum(input), output);

		let input = vec![-1, 0, 1, 0];
		let output: Vec<Vec<i32>> = vec![vec![-1, 0, 1]];
		assert_eq!(Solution::three_sum(input), output);

		let input = vec![0, 0, 0];
		let output: Vec<Vec<i32>> = vec![vec![0, 0, 0]];
		assert_eq!(Solution::three_sum(input), output);
	}
}
