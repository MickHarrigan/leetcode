use super::*;

#[test]
fn case1() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
}
#[test]
fn case2() {
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
}
#[test]
fn case3() {
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
}
#[test]
fn case4() {
    assert_eq!(Solution::min_operations(vec![3914], 3913), -1);
}
#[test]
fn case5() {
    assert_eq!(Solution::min_operations(vec![2431], 2432), -1);
}
#[test]
fn case6() {
    assert_eq!(Solution::min_operations(vec![8576], 8576), 1);
}
#[test]
fn case7() {
    assert_eq!(Solution::min_operations(vec![10, 1, 10, 10, 10], 40), 4);
}
#[test]
fn case8() {
    assert_eq!(Solution::min_operations(vec![1, 2, 3], 6), 3);
}
#[test]
fn case9() {
    assert_eq!(Solution::min_operations(vec![10, 1, 1, 1, 1, 1], 5), 5);
}
