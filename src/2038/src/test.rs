use super::*;

#[test]
fn case1() {
    let sol = Solution::winner_of_game("AAABABB".to_owned());
    assert_eq!(sol, true);
}

#[test]
fn case2() {
    let sol = Solution::winner_of_game("AA".to_owned());
    assert_eq!(sol, false);
}

#[test]
fn case3() {
    let sol = Solution::winner_of_game("ABBBBBBBBAAA".to_owned());
    assert_eq!(sol, false);
}
