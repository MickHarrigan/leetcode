struct Solution;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut candies = vec![1; ratings.len()];
        for ind in 0..ratings.len() {
            candies[ind] += Self::compare_recursive(&ratings, ind);
        }
        println!("{:?}", candies);
        candies.iter().sum()
    }

    fn compare_recursive(ratings: &Vec<i32>, ind: usize) -> i32 {
        // compare to neighbors
        // if greater than both
        // if not greater than both -> 1
        // if greater than prev -> prev + 1
        // if greater than next -> next + 1
        if ind != 0 && ratings[ind] > ratings[ind - 1] {
            let val = Self::compare_recursive(ratings, ind - 1) + 1;
            let next = if ind != ratings.len() - 1 {
                Self::compare_recursive(ratings, ind + 1) + 1
            } else {
                0
            };
            return val.min(next);
        } else if ind != ratings.len() - 1 && ratings[ind] > ratings[ind + 1] {
            Self::compare_recursive(ratings, ind + 1) + 1
        } else {
            0
        }
    }
}

fn main() {
    // let sol = Solution::candy(vec![1, 3, 2, 2, 1]);
    // let sol = Solution::candy(vec![2, 1, 0, 1]);
    // let sol = Solution::candy(vec![0, 1, 2, 3]);
    // let sol = Solution::candy(vec![1,0,2]);
    let sol = Solution::candy(vec![1, 6, 10, 8, 7, 3, 2]);
    println!("{:?}", sol);
}
