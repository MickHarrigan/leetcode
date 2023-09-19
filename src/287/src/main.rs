struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut low, mut high) = (1, nums.len() - 1);

        while low < high {
            let mid = (low + high) / 2;
            let mut count = 0;
            for num in nums.iter() {
                if *num <= mid as i32 {
                    count += 1;
                }
            }
            if count > mid {
                high = mid;
            } else {
                low = mid + 1;
            }
        }

        low as i32
    }
}

fn main() {
    let sol = Solution::find_duplicate(vec![1, 3, 4, 2, 2]);
    let sol = Solution::find_duplicate(vec![3, 1, 3, 4, 2]);
    println!("{:?}", sol);
}
