
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        // hash each number with the index as their value
        let mut hash: HashMap<i32, i32> = HashMap::new();
        for (k, v) in nums.iter().zip(0..) {
            match hash.get(&(target - k)) {
                Some(i) => return vec![v, *i],
                None => hash.insert(*k, v),
            };
        }
        vec![]
    }
}

fn main() {
    let sol = Solution::two_sum(vec![2, 7, 11, 15], 9);
    println!("{:?}", sol);
}
