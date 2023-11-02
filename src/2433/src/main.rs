struct Solution;

impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        std::iter::once(pref[0])
            .chain(pref.windows(2).map(|win| win[0] ^ win[1]))
            .collect()
        // let mut out = Vec::with_capacity(pref.len());
        // out.push(*pref.first().unwrap());
        // for i in 1..pref.len() {
        //     out.push(pref[i - 1] ^ pref[i]);
        // }
        // out
    }
}

fn main() {
    let sol = Solution::find_array(vec![5, 2, 0, 3, 1]);
    println!("{:?}", sol);
}

