/**
 * 2114. Maximum Number of Words Found in Sentences
 *
 * A sentence is a list of words that are separated
 * by a single space with no leading or trailing spaces.
 *
 * You are given an array of strings sentences,
 * where each sentences[i] represents a single sentence.
 *
 * Return the maximum number of words that appear in a single sentence.
 */
struct Solution;

impl Solution {
	pub fn most_words_found(sentences: Vec<String>) -> i32 {
		*sentences
			.iter()
			.map(|s| s.split(" ").collect::<Vec<&str>>().len())
			.collect::<Vec<usize>>()
			.iter()
			.max()
			.unwrap() as i32
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		let sentences = vec![
			String::from("alice and bob love leetcode"),
			String::from("i think so too"),
			String::from("this is great thanks very much"),
		];
		let output = 6;
		assert_eq!(Solution::most_words_found(sentences), output);

		let sentences = vec![
			String::from("please wait"),
			String::from("continue to fight"),
			String::from("continue to win"),
		];
		let output = 3;
		assert_eq!(Solution::most_words_found(sentences), output);
	}
}
