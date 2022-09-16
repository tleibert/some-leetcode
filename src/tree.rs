use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution();

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        root.and_then(|node| {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();

            node.borrow_mut().left = Self::invert_tree(right);
            node.borrow_mut().right = Self::invert_tree(left);
            Some(node)
        })
    }

    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = -1;
        let mut deepest: Option<Rc<RefCell<TreeNode>>> = None;

        fn grab_low(
            maybe_node: Option<Rc<RefCell<TreeNode>>>,
            current_depth: i32,
            max_depth: &mut i32,
            deepest: &mut Option<Rc<RefCell<TreeNode>>>,
        ) {
            if let Some(node) = maybe_node {
                if current_depth > *max_depth {
                    *max_depth = current_depth;
                    *deepest = Some(node.clone());
                }

                grab_low(
                    node.borrow_mut().left.take(),
                    current_depth + 1,
                    max_depth,
                    deepest,
                );
                grab_low(
                    node.borrow_mut().right.take(),
                    current_depth + 1,
                    max_depth,
                    deepest,
                );
            }
        }

        grab_low(root, 0, &mut max_depth, &mut deepest);
        deepest.unwrap().borrow().val
    }
}
