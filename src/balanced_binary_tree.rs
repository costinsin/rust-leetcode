use std::{cell::RefCell, cmp, rc::Rc};

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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn compute_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root_ref) = root {
            1 + cmp::max(
                compute_depth(&root_ref.borrow().left),
                compute_depth(&root_ref.borrow().right),
            )
        } else {
            0
        }
    }

    fn is_tree_balanced(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root_ref) = root {
            let left_depth = compute_depth(&root_ref.borrow().left);
            let right_depth = compute_depth(&root_ref.borrow().right);

            (left_depth - right_depth).abs() <= 1
                && is_tree_balanced(&root_ref.borrow().left)
                && is_tree_balanced(&root_ref.borrow().right)
        } else {
            true
        }
    }

    is_tree_balanced(&root)
}

fn main() {}
