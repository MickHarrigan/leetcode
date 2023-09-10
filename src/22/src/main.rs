struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::gen_parens(n as usize, 0, "".to_owned())
    }

    fn gen_parens(open: usize, close: usize, curr: String) -> Vec<String> {
        let mut res = Vec::new();
        if open == 0 && close == 0 {
            return vec![curr];
        }
        
        if open > 0 {
            res.append(&mut Self::gen_parens(open - 1, close + 1, curr.clone() + "(" ));
        }

        if close > 0 {
            res.append(&mut Self::gen_parens(open, close - 1, curr + ")" ));
        }
        res
    }
}

fn main() {
    let sol = Solution::generate_parenthesis(3);
    println!("{:?}", sol);
}
