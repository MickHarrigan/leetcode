struct Solution;

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 8 {
            (n - (n / 2)) * (n / 2)
        } else {
            match n % 3 {
                0 => 3_i32.pow((n / 3) as u32),
                1 => 4 * (3_i32.pow(((n - 4) / 3) as u32)),
                2 => 2 * (3_i32.pow(((n - 2) / 3) as u32)),
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let sol = Solution::integer_break(2);
    println!("{:?}", sol);
    let sol = Solution::integer_break(58);
    println!("{:?}", sol);
}

#[cfg(test)]
mod test;

