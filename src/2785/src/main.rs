struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut vowels = Vec::new();
        for ch in &mut bytes {
            if is_vowel(ch) {
                vowels.push(ch.clone());
            }
        }
        vowels.sort();
        let mut count = 0;
        for ch in &mut bytes {
            if is_vowel(ch) {
                *ch = vowels[count];
                count += 1;
            }
        }
        String::from_utf8(bytes).unwrap()
    }
}
fn is_vowel(c: &u8) -> bool {
    match c.to_ascii_lowercase() {
        b'a' | b'e' | b'i' | b'o' | b'u' => true,
        _ => false,
    }
}

fn main() {
    let sol = Solution::sort_vowels("lEetcOde".into());
    println!("{:?}", sol);
}

