struct Solution;

impl Solution {
    pub fn sort_by_bits(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| match a.count_ones().cmp(&b.count_ones()) {
            std::cmp::Ordering::Equal => a.cmp(&b),
            other => other,
        });
        arr
    }
}

fn main() {
    let sol = Solution::sort_by_bits(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    println!("{:?}", sol);
    let sol = Solution::sort_by_bits(vec![1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1]);
    println!("{:?}", sol);
}

