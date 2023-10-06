use super::*;

const VALUES_8: [i32; 7] = [1, 2, 4, 6, 9, 12, 18];
const VALUES_OTHER: [i32; 9] = [18, 27, 36, 54, 81, 108, 162, 243, 324];

#[test]
fn test_nlt8() {
    for i in 2..8 {
        assert_eq!(Solution::integer_break(i as i32), VALUES_8[i - 2]);
    }
}

#[test]
fn test_ngt8() {
    for i in 8..=16 {
        assert_eq!(Solution::integer_break(i as i32), VALUES_OTHER[i - 8]);
    }
}
