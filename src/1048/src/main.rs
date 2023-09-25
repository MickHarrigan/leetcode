use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        // start with the first word
        // if all the chars of the first word appear in the correct order in the next word add that
        // to the chain and recurse with that
        // otherwise move to the next test word
        let mut words = words;
        words.sort_by_key(|a| a.len());

        // rustier way to do this
        words
            .into_iter()
            .scan(HashMap::new(), |map, word| {
                let len = (0..word.len())
                    .filter_map(|i| map.get(&[&word[..i], &word[i + 1..]].concat()))
                    .copied()
                    .max()
                    .unwrap_or(0);
                map.insert(word, len + 1);
                Some(len + 1)
            })
            .max()
            .unwrap_or(1)
    }
}

fn main() {
    let sol = Solution::longest_str_chain(vec![]);
    println!("{:?}", sol);
}
