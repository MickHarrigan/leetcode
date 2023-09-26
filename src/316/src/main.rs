struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        use std::collections::{HashMap, HashSet};
        let mut seen: HashSet<char> = HashSet::new();
        let mut last_occ: HashMap<char, usize> = HashMap::new();
        let mut stack = Vec::new();
        for (ind, ch) in s.char_indices() {
            last_occ.insert(ch, ind);
        }

        s.char_indices().for_each(|(ind, ch)| {
            if !seen.contains(&ch) {
                while let Some(&last) = stack.last() {
                    if ch < last && ind < *last_occ.get(&last).unwrap() {
                        seen.remove(&stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                seen.insert(ch);
                stack.push(ch);
            }
        });
        stack.into_iter().collect()
    }
}

fn main() {
    let sol = Solution::remove_duplicate_letters("cbacdcbc".to_owned());
    // let sol = Solution::remove_duplicate_letters("bcabc".to_owned());
    println!("{:?}", sol);
}
