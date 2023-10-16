struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut dp: Vec<Vec<i32>> = Vec::with_capacity(34);
        for i in 0..=row_index as usize {
            if i == 0 {
                dp.push(vec![1]);
            } else if i == 1 {
                dp.push(vec![1, 1]);
            } else {
                dp.push(Self::next_row(&dp[i - 1]));
            }
        }
        dp[row_index as usize].clone()
    }
    pub fn next_row(prev: &Vec<i32>) -> Vec<i32> {
        let mut out = Vec::with_capacity(prev.len() + 1);
        out.push(1);
        for i in 1..prev.len() {
            out.push(prev[i] + prev[i - 1]);
        }
        out.push(1);
        out
    }
}

fn main() {
    let sol = Solution::get_row(4);
    println!("{:?}", sol);
}

