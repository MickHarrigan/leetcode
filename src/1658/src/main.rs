struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let ans: i32 = nums.iter().sum::<i32>() - x;
        if ans == 0 {
            return nums.len() as i32;
        }
        let (mut sum, mut len, mut left) = (0, 0, 0);
        for right in 0..nums.len() {
            sum += nums[right];
            while left <= right as i32 && sum > ans {
                sum -= nums[left as usize];
                left += 1;
            }
            if sum == ans {
                len = len.max(right as i32 - left + 1);
            }
        }
        if len != 0 {
            return nums.len() as i32 - len;
        }
        -1i32
    }
}

fn main() {
    let sol = Solution::min_operations(vec![1, 1, 4, 2, 3], 5);
    println!("{:?}", sol);
}

#[cfg(test)]
mod test;
