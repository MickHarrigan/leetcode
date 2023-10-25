struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        // n can be ignored
        ((k - 1).count_ones() % 2) as i32
    }
}

fn main() {
    let sol = Solution::kth_grammar(2, 7);
    println!("{:?}", sol);
}

