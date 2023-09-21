struct Solution;

impl Solution {
    // solution is to merge the arrarys lazily (iterators)
    // then find the median from that
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        let mut iter = Self::merge(&nums1, &nums2);
        match (total / 2, total % 2) {
            (0, 0) => 0f64,
            (0, 1) => iter.next().unwrap() as f64,
            (n, 1) => iter.nth(n).unwrap() as f64,
            (n, _) => {
                let mut item = iter.skip(n - 1);
                (item.next().unwrap() + item.next().unwrap()) as f64 / 2f64
            }
        }
    }
    fn merge<'a>(nums1: &'a Vec<i32>, nums2: &'a Vec<i32>) -> impl Iterator<Item = i32> + 'a {
        use std::iter::from_fn;
        let mut iter1 = nums1.iter().copied().peekable();
        let mut iter2 = nums2.iter().copied().peekable();
        from_fn(move || match (iter1.peek(), iter2.peek()) {
            (None, None) => None,
            (Some(_), None) => iter1.next(),
            (None, Some(_)) => iter2.next(),
            (Some(a), Some(b)) if a <= b => iter1.next(),
            (Some(_a), Some(_b)) => iter2.next(),
        })
    }
}

fn main() {
    let sol = Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
    let sol = Solution::find_median_sorted_arrays(vec![], vec![1, 2, 3]);
    println!("{:?}", sol);
}
