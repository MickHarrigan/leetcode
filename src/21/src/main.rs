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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut l), Some(mut r)) => {
                if l.val < r.val {
                    l.next = Solution::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Solution::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
            (None, None) => None,
            (Some(a), None) | (None, Some(a)) => Some(a),
        }
    }
}

fn main() {
    let mut l1 = ListNode::new(1);
    l1.next = Some(Box::new(ListNode::new(2)));
    l1.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    let l1 = Some(Box::new(l1));

    let mut l2 = ListNode::new(1);
    l2.next = Some(Box::new(ListNode::new(3)));
    l2.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    let l2 = Some(Box::new(l2));

    let sol = Solution::merge_two_lists(l1, l2);
    println!("{:?}", sol);
}
