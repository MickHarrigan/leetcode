struct Solution;

impl Solution {
    #[cfg(not(feature = "rusty"))]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return 0;
    }
    #[cfg(feature = "rusty")]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        return 0;
    }
}

fn main() {}

#[cfg(test)]
mod test;
