struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut a = Vec::new();
        for ch in s.chars() {
            if ch == '#' {
                a.pop();
            } else {
                a.push(ch);
            }
        }
        let mut b = Vec::new();
        for ch in t.chars() {
            if ch == '#' {
                b.pop();
            } else {
                b.push(ch);
            }
        }

        a == b
    }
}

fn main() {
    let sol = Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string());
    println!("{:?}", sol);
}

