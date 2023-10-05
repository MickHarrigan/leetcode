struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hash = HashMap::new();
        let n = nums.len();
        let mut out = Vec::new();

        nums.iter().for_each(|num| {
            hash.entry(num).and_modify(|count| *count += 1).or_insert(1);
        });
        hash.iter().for_each(|(k, v)| {
            if *v > (n / 3) {
                out.push(**k);
            }
        });
        out
    }
}

fn main() {
    let sol = Solution::majority_element(vec![2, 2]);
    println!("{:?}", sol);
}

