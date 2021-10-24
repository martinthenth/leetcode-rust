/**
 * [1314] Matrix Block Sum
 *
 * Given a m x n matrix mat and an integer k, return a matrix answer
 * where each answer[i][j] is the sum of all elements mat[r][c] for:
 *
 * i - k <= r <= i + k,
 * j - k <= c <= j + k, and
 * (r, c) is a valid position in the matrix.
 */
struct Solution;

impl Solution {
	pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
		use std::cmp;

		let m = mat.len();
		let n = mat[0].len();
		let mut i = 0;
		let mut j = 0;

		// First, compute a prefix sum array.
		let mut prefix_sum_matrix = vec![vec![0; n + 1]; m + 1];

		while i < m {
			while j < n {
				prefix_sum_matrix[i + 1][j + 1] =
					mat[i][j] + prefix_sum_matrix[i + 1][j] + prefix_sum_matrix[i][j + 1]
						- prefix_sum_matrix[i][j];

				j += 1;
			}

			j = 0;
			i += 1;
		}

		// Second, compute a block sum array.
		let mut block_sum_matrix = mat.to_vec();

		i = 0;
		j = 0;

		while i < m {
			while j < n {
				let row0 = cmp::max(0, i as i32 - k) as usize;
				let col0 = cmp::max(0, j as i32 - k) as usize;
				let row1 = cmp::min(m - 1, i + k as usize);
				let col1 = cmp::min(n - 1, j + k as usize);

				block_sum_matrix[i][j] = prefix_sum_matrix[row1 + 1][col1 + 1]
					- prefix_sum_matrix[row0][col1 + 1]
					- prefix_sum_matrix[row1 + 1][col0]
					+ prefix_sum_matrix[row0][col0];

				j += 1;
			}

			j = 0;
			i += 1;
		}

		block_sum_matrix
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
		let k = 1;
		let result = vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]];
		assert_eq!(Solution::matrix_block_sum(mat, k), result);

		let mat = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
		let k = 2;
		let result = vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]];
		assert_eq!(Solution::matrix_block_sum(mat, k), result);
	}
}
