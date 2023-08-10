use super::*;

#[test]
fn empty_vec() {
    assert_eq!(Solution::search(vec![], 0), -1);
}

#[test]
fn single_item() {
    assert_eq!(Solution::search(vec![1], 1), 0);
}

#[test]
fn single_item_wrong() {
    // this is also case 3
    assert_eq!(Solution::search(vec![0], 1), -1);
}

#[test]
fn case1() {
    let mut vector = (0..=7).collect::<Vec<i32>>();
    vector.remove(3);
    vector.rotate_right(4);
    println!("{:?}", vector);
    assert_eq!(Solution::search(vector, 0), 4);
}

#[test]
fn case2() {
    let mut vector = (0..=7).collect::<Vec<i32>>();
    vector.remove(3);
    vector.rotate_right(4);
    assert_eq!(Solution::search(vector, 3), -1);
}

#[test]
fn unsorted_short() {
    assert_eq!(Solution::search(vec![5,1,3], 3), 2);
}

