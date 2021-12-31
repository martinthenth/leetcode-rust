/**
 * [278] First Bad Version
 *
 * You are a product manager and currently leading a team to develop a new product.
 * Unfortunately, the latest version of your product fails the quality check.
 * Since each version is developed based on the previous version,
 * all the versions after a bad version are also bad.
 *
 * Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one,
 * which causes all the following ones to be bad.
 *
 * You are given an API bool isBadVersion(version) which returns whether version is bad.
 * Implement a function to find the first bad version.
 * You should minimize the number of calls to the API.
 */
struct Solution {
	bad: i32,
}

impl Solution {
	fn new(bad: i32) -> Self {
		Solution { bad }
	}

	#[allow(non_snake_case)]
	fn isBadVersion(&self, version: i32) -> bool {
		version >= self.bad
	}
}

impl Solution {
	pub fn first_bad_version(&self, n: i32) -> i32 {
		let mut low = 1;
		let mut high = n;

		while low < high {
			let mid = low + (high - low) / 2;

			if self.isBadVersion(mid) {
				high = mid;
			} else {
				low = mid + 1
			}
		}

		low
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let n = 5;
		let bad = 4;
		let solution = Solution::new(bad);
		assert_eq!(solution.first_bad_version(n), bad);

		let n = 1;
		let bad = 1;
		let solution = Solution::new(bad);
		assert_eq!(solution.first_bad_version(n), bad);

		let n = 2126753390;
		let bad = 1702766719;
		let solution = Solution::new(bad);
		assert_eq!(solution.first_bad_version(n), bad);
	}
}
