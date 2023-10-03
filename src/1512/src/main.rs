struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        // hash with k = nums[n], v = index
        use std::collections::HashMap;

        let mut hash: HashMap<&i32, Vec<usize>> = HashMap::new();

        nums.iter().enumerate().for_each(|(ind, num)| {
            hash.entry(num)
                .and_modify(|k| k.push(ind))
                .or_insert(vec![ind]);
        });

        hash.iter()
            .fold(0, |acc, (_k, v)| acc + (v.len() * (v.len() - 1) / 2)) as i32
    }
}

fn main() {
    let sol = Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]);
    println!("{:?}", sol);
}

