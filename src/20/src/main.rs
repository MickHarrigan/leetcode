struct Solution;

#[derive(Eq, PartialEq)]
enum Bracket {
    Parentheses,
    Square,
    Curly,
    Empty,
}

trait BracketChar {
    fn from_char(c: char) -> Option<Self>
    where
        Self: Sized;
}

impl BracketChar for Bracket {
    fn from_char(c: char) -> Option<Self>
    where
        Self: Sized,
    {
        match c {
            '(' | ')' => Some(Bracket::Parentheses),
            '[' | ']' => Some(Bracket::Square),
            '{' | '}' => Some(Bracket::Curly),
            _ => None,
        }
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // create a stack that pushes when an open comes and pops if close
        // if there is ever a break early return false
        let mut stack: Vec<_> = Vec::with_capacity(s.len());
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => stack.push(Bracket::from_char(ch).unwrap()),
                ')' | ']' | '}' => {
                    if stack.pop() != Bracket::from_char(ch) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        stack.len() == 0
    }
}

fn main() {
    let sol = Solution::is_valid("({})".to_string());
    println!("{:?}", sol);
}
