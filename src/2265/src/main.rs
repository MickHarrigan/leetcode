struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // each leaf is +1 to the output,
        // each middle node only is added when its children average to itself

        // first check self
        // then check children
        // return value upwards

        match root {
            Some(node) => {
                let mut stack = Vec::new();
                let mut out = 0;
                stack.push(node);
                while !stack.is_empty() {
                    let curr = stack.pop();
                    let (size, sum) = Self::average_rec(&curr);
                    if curr.is_some() {
                        if let Some(left) = curr.clone().unwrap().borrow().left.clone() {
                            stack.push(left);
                        }
                        if let Some(right) = curr.clone().unwrap().borrow().right.clone() {
                            stack.push(right);
                        }
                    }
                    if curr.is_some() && sum / size == curr.unwrap().borrow().val {
                        out += 1;
                    }
                }
                out
            }
            None => 0,
        }
    }
    /// returns a (size of subtree: i32, sum of values: i32)
    fn average_rec(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = node {
            let mut sum = 0;
            let mut size = 1;
            let mut stack = Vec::new();
            stack.push(node.clone());
            while !stack.is_empty() {
                let curr = stack.pop().unwrap();
                sum += curr.borrow().val;
                if let Some(left) = curr.borrow().left.clone() {
                    stack.push(left);
                    size += 1;
                }
                if let Some(right) = curr.borrow().right.clone() {
                    stack.push(right);
                    size += 1;
                };
            }

            (size, sum)
        } else {
            (0, 0)
        }
    }
}

fn main() {
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.clone().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    // root.clone().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));

    let sol = Solution::average_of_subtree(root);
    println!("{:?}", sol);
}

