struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s.split(' ')
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

fn main() {
    let sol = Solution::reverse_words("Let's take LeetCode contest".to_owned());
    println!("{:?}", sol);
}

