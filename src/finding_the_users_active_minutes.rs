/**
 * 1817. Finding the Users Active Minutes
 *
 * You are given the logs for users' actions on LeetCode, and an integer k.
 * The logs are represented by a 2D integer array logs where each logs[i] = [IDi, timei]
 * indicates that the user with IDi performed an action at the minute timei.
 *
 * Multiple users can perform actions simultaneously,
 * and a single user can perform multiple actions in the same minute.
 *
 * The user active minutes (UAM) for a given user is defined as the number of unique minutes
 * in which the user performed an action on LeetCode. A minute can only be counted once,
 * even if multiple actions occur during it.
 *
 * You are to calculate a 1-indexed array answer of size k such that, for each j (1 <= j <= k),
 * answer[j] is the number of users whose UAM equals j.
 *
 * Return the array answer as described above.
 */
struct Solution;

impl Solution {
	pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
		use std::collections::HashMap;

		let mut result = vec![0; k as usize];
		let mut hash_map: HashMap<i32, Vec<i32>> = HashMap::new();

		for log in logs {
			let user_id = log[0];
			let minute = log[1];

			hash_map.entry(user_id).or_default().push(minute)
		}

		for (_key, mut value) in hash_map {
			value.sort();
			value.dedup();

			result[value.len() - 1] += 1;
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let logs = vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]];
		let k = 5;
		let output = vec![0, 2, 0, 0, 0];
		assert_eq!(Solution::finding_users_active_minutes(logs, k), output);

		let logs = vec![vec![1, 1], vec![2, 2], vec![2, 3]];
		let k = 4;
		let output = vec![1, 1, 0, 0];
		assert_eq!(Solution::finding_users_active_minutes(logs, k), output);
	}
}
