/**
 * 1672. Richest Customer Wealth
 *
 * You are given an m x n integer grid accounts where accounts[i][j] is the amount of money
 * the i​​​​​​​​​​​th​​​​ customer has in the j​​​​​​​​​​​th​​​​ bank. Return the wealth that the richest customer has.
 *
 * A customer's wealth is the amount of money they have in all their bank accounts.
 * The richest customer is the customer that has the maximum wealth.
 */
struct Solution;

impl Solution {
	pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
		use std::cmp;

		let mut max = 0;
		let mut i = 0;
		let m = accounts.len();

		while i < m {
			let value = accounts[i].iter().sum();

			max = cmp::max(max, value);
			i += 1;
		}

		max
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let matrix = vec![vec![1, 2, 3], vec![3, 2, 1]];
		assert_eq!(Solution::maximum_wealth(matrix), 6);

		let matrix = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
		assert_eq!(Solution::maximum_wealth(matrix), 10);

		let matrix = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
		assert_eq!(Solution::maximum_wealth(matrix), 17);
	}
}
