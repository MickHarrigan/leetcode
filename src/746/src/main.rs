struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = Vec::with_capacity(cost.len());
        for (ind, each) in cost.iter().enumerate() {
            let min_prev = match ind {
                0 | 1 => 0,
                _ => dp[ind - 1].min(dp[ind - 2]),
            };
            dp.push(each + min_prev);
        }
        dp[cost.len() - 1].min(dp[cost.len() - 2])
    }
}

fn main() {
    let sol = Solution::min_cost_climbing_stairs(vec![10, 15, 20]);
    println!("{:?}", sol);
    let sol = Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1]);
    println!("{:?}", sol);
}

