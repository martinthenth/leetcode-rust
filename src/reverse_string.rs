/**
 * [344] Reverse String
 *
 * Write a function that reverses a string. The input string is given as an array of characters s.
 */
struct Solution;

impl Solution {
	pub fn reverse_string(s: &mut Vec<char>) {
		let mut i = 0;
		let mut j = s.len() - 1;

		while i < j {
			let temp = s[i];

			s[i] = s[j];
			s[j] = temp;

			i += 1;
			j -= 1;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let mut s = vec!['h', 'e', 'l', 'l', 'o'];
		let result = vec!['o', 'l', 'l', 'e', 'h'];
		Solution::reverse_string(&mut s);
		assert_eq!(s, result);

		let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
		let result = vec!['h', 'a', 'n', 'n', 'a', 'H'];
		Solution::reverse_string(&mut s);
		assert_eq!(s, result);

		let mut s = vec!['H'];
		let result = vec!['H'];
		Solution::reverse_string(&mut s);
		assert_eq!(s, result);

		let mut s = vec!['H', 'a'];
		let result = vec!['a', 'H'];
		Solution::reverse_string(&mut s);
		assert_eq!(s, result);
	}
}
