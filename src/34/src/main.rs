struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let l = nums.partition_point(|x| x < &target);
        let r = nums.partition_point(|x| x <= &target) - 1;

        vec![
            match nums.get(l) == Some(&target) {
                true => l as i32,
                false => -1,
            },
            match nums.get(r) == Some(&target) {
                true => r as i32,
                false => -1,
            },
        ]
    }
}

fn main() {
    let sol = Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8);
    println!("{:?}", sol);
}

