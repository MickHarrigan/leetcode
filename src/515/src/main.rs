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
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // track depth with bfs
        let mut out: Vec<i32> = Vec::new();
        Self::largest_values_helper(root, &mut out, 0);
        out
    }

    fn largest_values_helper(node: Option<Rc<RefCell<TreeNode>>>, out: &mut Vec<i32>, depth: usize) {
        use std::collections::VecDeque;
        // bfs non-recursively

        if let Some(node) = node {
            let mut queue = VecDeque::new();
            queue.push_back((node, depth));

            while queue.len() > 0 {
                let (item, depth) = queue.pop_front().unwrap();
                if item.borrow().left.is_some() {
                    queue.push_back((item.borrow().left.clone().unwrap(), depth + 1));
                }
                if item.borrow().right.is_some() {
                    queue.push_back((item.borrow().right.clone().unwrap(), depth + 1));
                }

                // if the node exists,
                // set out[depth] to max(out[depth], a.val)
                match out.get_mut(depth) {
                    Some(b) => *b = *b.max(&mut item.borrow_mut().val),
                    None => out.push(item.borrow().val),
                }
            }
        }
    }
}

fn main() {
    let tree = Rc::new(RefCell::new(TreeNode::new(0)));
    tree.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    tree.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    let sol = Solution::largest_values(Some(tree));
    println!("{:?}", sol);
}

