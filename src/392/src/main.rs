struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut t = t.chars().peekable();
        let mut s = s.chars().peekable();
        loop {
            match (s.peek(), t.next()) {
                (Some(small), Some(large)) if small == &large => s.next(),
                (Some(_), Some(_)) => None,
                (Some(_), None) => return false,
                _ => return true,
            };
        }
    }
}

fn main() {
    let sol = Solution::is_subsequence("axc".to_owned(), "ahbgdc".to_owned());
    println!("{:?}", sol);
}
