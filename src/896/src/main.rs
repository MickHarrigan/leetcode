struct Solution;

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.as_slice()
            .windows(2)
            .map(|a| a[0].cmp(&a[1]))
            .filter(|a| a.is_ne())
            .fold((true, None), |acc, x| {
                if let Some(prev) = acc.1 {
                    (acc.0 && (prev == x), Some(x))
                } else {
                    (true, Some(x))
                }
            })
            .0
    }
}

fn main() {
    let sol = Solution::is_monotonic(vec![1, 1, 0, 1]);
    println!("{:?}", sol);
    let sol = Solution::is_monotonic(vec![6, 5, 4, 4]);
    println!("{:?}", sol);
    let sol = Solution::is_monotonic(vec![6]);
    println!("{:?}", sol);
}
