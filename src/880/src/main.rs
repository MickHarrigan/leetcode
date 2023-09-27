struct Solution;

impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut k = k as i64;
        let mut out_len: i64 = 0;

        let mut ind = 0;

        while out_len < k {
            if let Some(ch) = s.chars().nth(ind) {
                match ch.is_digit(10) {
                    true => out_len *= ch.to_digit(10).unwrap() as i64,
                    false => out_len += 1,
                }
            }
            ind += 1;
        }

        for i in (0..ind).rev() {
            if let Some(ch) = s.chars().nth(i) {
                match ch.is_digit(10) {
                    true => {
                        out_len /= ch.to_digit(10).unwrap() as i64;
                        k %= out_len;
                    }
                    false => {
                        if k == 0 || k == out_len {
                            return ch.to_string();
                        }
                        out_len -= 1;
                    }
                }
            }
        }
        unreachable!();
    }
}

fn main() {
    let sol = Solution::decode_at_index("leet2code3".to_string(), 10);
    println!("{:?}", sol);
}
