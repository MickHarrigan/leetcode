struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 {
            let word = x.to_string();
            return word
                .chars()
                .zip(word.chars().rev())
                .take(word.len() / 2)
                .all(|(l, r)| l == r);
        }
        false
    }
}

fn main() {
    let sol = Solution::is_palindrome(333);
    println!("{:?}", sol);
}

#[cfg(test)]
mod test;
