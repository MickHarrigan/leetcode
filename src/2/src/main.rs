// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (Some(left), Some(right)) => {
                let sum = left.val + right.val;
                if sum > 9 {
                    let carry = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(
                            Solution::add_two_numbers(carry, left.next),
                            right.next,
                        ),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(left.next, right.next),
                    }))
                }
            }
            (None, Some(x)) | (Some(x), None) => Some(x),
            (None, None) => None,
        }
    }
}

fn main() {
    let l1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 2, next: None })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    }));

    let sol = Solution::add_two_numbers(l1, l2);
    println!("value of sol = {:?}", sol);
}
