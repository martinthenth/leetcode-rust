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

		let mut hash_map = HashMap::new();
		let mut hash_set = HashSet::new();
		let mut i = 0;
		let mut j;

		// Seed hash map
		while i < nums.len() {
			hash_map.insert(nums[i], i);

			i += 1;
		}

		// Find pairs
		i = 0;

		while i < nums.len() - 1 {
			j = i + 1;

			while j < nums.len() {
				let diff = -1 * (nums[i] + nums[j]);

				match hash_map.get(&diff) {
					Some(&index) => {
						if index > j {
							let mut vector = vec![nums[i], nums[j], diff];

							vector.sort();
							hash_set.insert(vector);
						}
					}
					None => {}
				}

				j += 1;
			}

			i += 1;
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
		let output = Solution::three_sum(input);
		let vec1 = vec![-1, -1, 2];
		let vec2 = vec![-1, 0, 1];
		println!("{:?}", output);
		assert!(output.contains(&vec1));
		assert!(output.contains(&vec2));
		assert!(output.len() == 2);

		let input = vec![];
		let output = Solution::three_sum(input);
		let vec1: Vec<Vec<i32>> = vec![];
		assert_eq!(output, vec1);

		let input = vec![0];
		let output = Solution::three_sum(input);
		let vec1: Vec<Vec<i32>> = vec![];
		assert_eq!(output, vec1);

		let input = vec![-1, 0, 1, 0];
		let output = Solution::three_sum(input);
		let vec1 = vec![-1, 0, 1];
		assert!(output.contains(&vec1));
		assert!(output.len() == 1);

		let input = vec![0, 0, 0];
		let output = Solution::three_sum(input);
		let vec1 = vec![0, 0, 0];
		assert!(output.contains(&vec1));
	}
}
