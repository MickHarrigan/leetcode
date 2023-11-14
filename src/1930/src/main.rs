struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let mut letter_locations: HashMap<char, Vec<usize>> = HashMap::new();
        let mut out: HashSet<String> = HashSet::new();

        for (ind, ch) in s.char_indices() {
            letter_locations
                .entry(ch)
                .and_modify(|list| list.push(ind))
                .or_insert(vec![ind]);
        }
        for (k, v) in letter_locations.iter() {
            match v.len() {
                1 => {
                    // ignored as it can only be a center
                }
                a => {
                    if a > 2 {
                        out.insert(format!("{k}{k}{k}"));
                    }
                    let low = v.first().unwrap();
                    let high = v.last().unwrap();
                    for (key, val) in letter_locations.iter() {
                        // if there is a val that falls between low and high add the string to the
                        // output
                        if val.iter().any(|x| x < &&high && x > &&low) {
                            out.insert(format!("{k}{key}{k}"));
                        }
                    }
                }
            }
        }

        // This is the old and slow version that failed on leetcode
        //
        // for (ind, ch) in s.char_indices().skip(1) {
        //     // check each index,
        //     // then check each side of it for a char that can be repeated on either side
        //     // if it exists try to add it to the set

        //     for i in 0..ind {
        //         let left = s.chars().nth(i).unwrap();
        //         if letter_locations
        //             .get(&left)
        //             .unwrap()
        //             .iter()
        //             .any(|x| x > &ind)
        //         {
        //             out.insert(format!("{}{}{}", left, ch, left));
        //         }
        //     }
        // }

        out.len() as i32
    }
}

fn main() {
    let sol = Solution::count_palindromic_subsequence("bbcbaba".into());
    println!("{:?}", sol);
}

