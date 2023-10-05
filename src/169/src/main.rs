struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut hash: HashMap<i32, usize> = HashMap::new();
        let mut max = (i32::MIN, usize::MIN);
        nums.iter().for_each(|num| {
            let val = hash.entry(*num).and_modify(|e| *e += 1).or_insert(1);
            if *val > max.1 {
                max = (*num, *val);
            }
        });
        max.0
    }
}

fn main() {
    let sol = Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]);
    println!("{:?}", sol);
}

