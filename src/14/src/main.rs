struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter().reduce(|acc, curr| {
            acc.chars()
                .zip(curr.chars())
                .take_while(|(a, c)| a == c)
                .map(|(a, _)| a)
                .collect()
        }).unwrap()
    }
}

fn main() {
    let sol = Solution::longest_common_prefix(vec![]);
    println!("{:?}", sol);
}
