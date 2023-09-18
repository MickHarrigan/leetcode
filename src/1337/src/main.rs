struct Solution;

impl Solution {
    fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // for row in mat count the number of 1s and pair that with its index
        // something...
        // take(k) and return that
        let mut out: Vec<_> = mat
            .iter()
            .enumerate()
            .map(|(ind, row)| (ind, row.iter().sum::<i32>()))
            .collect();

        out.sort_by(|a, b| a.1.cmp(&b.1));
        out.iter()
            .map(|(a, _)| *a as i32)
            .take(k as usize)
            .collect()
    }
}

fn main() {
    let mat = vec![
        vec![1, 0, 0, 0],
        vec![1, 1, 1, 1],
        vec![1, 0, 0, 0],
        vec![1, 0, 0, 0],
    ];
    let sol = Solution::k_weakest_rows(mat, 2);
    println!("{:?}", sol);
}
