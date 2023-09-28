struct Solution;

impl Solution {
    pub fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
        // binary search?
        let mut out = nums;
        out.sort_by(|a, b| (a % 1).cmp(&(b % 1)));
        out
    }
}

fn main() {
    let sol = Solution::sort_array_by_parity(vec![3, 1, 2, 4]);
    println!("{:?}", sol);
}

