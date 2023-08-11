use super::*;

#[test]
fn case1() {
    assert_eq!(Solution::is_palindrome(333), true);
}
#[test]
fn case2() {
    assert_eq!(Solution::is_palindrome(-121), false);
}
#[test]
fn case3() {
    assert_eq!(Solution::is_palindrome(10), false);
}
#[test]
fn case4() {
    assert_eq!(Solution::is_palindrome(123), false);
}
#[test]
fn case5() {
    assert_eq!(Solution::is_palindrome(121), true);
}

