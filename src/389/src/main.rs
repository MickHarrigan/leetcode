struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        use std::collections::HashMap;
        let mut hash = HashMap::new();
        s.chars().for_each(|ch| *hash.entry(ch).or_insert(1) += 1);
        // then remove each from t or return what is left in the hash
        for ch in t.chars() {
            match hash.get_mut(&ch) {
                Some(a) if *a == 1 => return ch,
                Some(a) => *a -= 1,
                None => return ch,
            }
        }
        unreachable!();
    }
}

fn main() {
    let sol = Solution::find_the_difference("a".to_owned(), "aa".to_owned());
    println!("{:?}", sol);
}
