struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;
        let mut out = 0;
        let mut hash: HashMap<char, i32> = HashMap::new();
        let mut low = -1;

        for (ch, ind) in s.chars().zip(0..) {
            if let Some(val) = hash.insert(ch, ind) {
                low = low.max(val);
            }
            out = out.max(ind - low);
        }

        return out;

    }
}

fn main() {
    let s = "pwwkew".to_string();
    let sol = Solution::length_of_longest_substring(s);
    println!("{:?}", sol);
}
