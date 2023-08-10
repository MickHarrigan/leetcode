struct Solution;

impl Solution {
    #[cfg(not(feature = "rusty"))]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }

        let mut low = 0;
        let mut high = nums.len() - 1;

        while low <= high {
            let mid = (high + low) / 2;
            if nums[mid] == target {
                return mid as i32;
            }

            if nums[low] <= nums[mid] {
                if nums[low] <= target && nums[mid] > target {
                    high = mid - 1;
                } else {
                    low = mid + 1;
                }
            } else {
                if nums[mid] < target && nums[high] >= target {
                    low = mid + 1;
                } else {
                    high = mid - 1;
                }
            }
        }

        return -1;
    }
    #[cfg(feature = "rusty")]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        todo!();
    }
}

fn main() {
    let sol = Solution::search(vec![5,1,3], 3);
    println!("solution here is {}", sol);
}

#[cfg(test)]
mod test;
