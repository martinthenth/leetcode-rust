/**
 * [557] Reverse Words in a String III
 *
 * Given a string s, reverse the order of characters in each word within
 * a sentence while still preserving whitespace and initial word order.
 */
struct Solution;

impl Solution {
	pub fn reverse_words(s: String) -> String {
		let words = s.split(" ");
		let mut list = vec![];

		for word in words {
			let mut chars: Vec<char> = word.chars().collect();
			let mut i = 0;
			let mut j = chars.len();

			while i < j {
				let temp = chars[i];

				chars[i] = chars[j - 1];
				chars[j - 1] = temp;

				i += 1;
				j -= 1;
			}

			let word: String = chars.into_iter().collect();

			list.push(word)
		}

		list.join(" ")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let s = String::from("Let's take LeetCode contest");
		let result = String::from("s'teL ekat edoCteeL tsetnoc");
		assert_eq!(Solution::reverse_words(s), result);

		let s = String::from("God Ding");
		let result = String::from("doG gniD");
		assert_eq!(Solution::reverse_words(s), result);

		let s = String::from("G");
		let result = String::from("G");
		assert_eq!(Solution::reverse_words(s), result);
	}
}
