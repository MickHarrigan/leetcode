struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        // take the level below it and operate on that
        if n > 1 {
            let mut res: Vec<(_, usize)> = Vec::new();
            let prev = Self::count_and_say(n - 1);
            for num in prev.chars() {
                match res.last_mut() {
                    Some((val, count)) if val == &num => *count += 1,
                    _ => res.push((num, 1)),
                }
            }
            res.iter()
                .map(|(ch, cnt)| format!("{}{}", cnt, ch))
                .collect()
        } else {
            "1".to_owned()
        }
    }
}

fn main() {
    let sol = Solution::count_and_say(5);
    println!("{:?}", sol);
}
