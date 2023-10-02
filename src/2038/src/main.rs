struct Solution;

impl Solution {
    pub fn winner_of_game(colors: String) -> bool {
        if colors.len() < 3 {
            return false;
        }
        let (mut a, mut b) = (0, 0);
        colors.as_bytes().windows(3).for_each(|win| {
            match win[0] {
                b'A' if win[0] == win[1] && win[1] == win[2] => {
                    a += 1;
                }
                b'B' if win[0] == win[1] && win[1] == win[2] => {
                    b += 1;
                }
                _ => {}
            };
        });

        return a > b;
    }
}

fn main() {
    let sol = Solution::winner_of_game("".to_owned());
    println!("{:?}", sol);
}

#[cfg(test)]
mod test;

