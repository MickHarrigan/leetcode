struct Solution;

impl Solution {
    pub fn letter_combinations(dig: String) -> Vec<String> {
        if dig == "" {
            return vec![];
        }
        Self::gen_letters(dig.chars().rev().collect(), "".to_owned())
    }

    fn gen_letters(mut start: Vec<char>, curr: String) -> Vec<String> {
        let mut res = Vec::new();
        if let Some(ch) = start.pop() {
            match ch {
                '2' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "a"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "b"));
                    res.append(&mut Self::gen_letters(start, curr + "c"));
                    res
                }
                '3' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "d"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "e"));
                    res.append(&mut Self::gen_letters(start, curr + "f"));
                    res
                }
                '4' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "g"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "h"));
                    res.append(&mut Self::gen_letters(start, curr + "i"));
                    res
                }
                '5' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "j"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "k"));
                    res.append(&mut Self::gen_letters(start, curr + "l"));
                    res
                }
                '6' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "m"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "n"));
                    res.append(&mut Self::gen_letters(start, curr + "o"));
                    res
                }
                '7' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "p"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "q"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "r"));
                    res.append(&mut Self::gen_letters(start, curr + "s"));
                    res
                }
                '8' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "t"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "u"));
                    res.append(&mut Self::gen_letters(start, curr + "v"));
                    res
                }
                '9' => {
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "w"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "x"));
                    res.append(&mut Self::gen_letters(start.clone(), curr.clone() + "y"));
                    res.append(&mut Self::gen_letters(start, curr + "z"));
                    res
                }
                _ => {
                    vec![]
                }
            }
        } else {
            vec![curr]
        }
    }
}

fn main() {
    let sol = Solution::letter_combinations("".to_owned());
    println!("{:?}", sol);
}
