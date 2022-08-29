use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

struct Solution();

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

#[derive(Eq)]
struct Wrapper(Box<ListNode>);

impl Ord for Wrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.val.cmp(&other.0.val)
    }
}

impl PartialOrd for Wrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.val.cmp(&other.0.val))
    }
}

impl PartialEq for Wrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.val == other.0.val
    }
}

impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<Wrapper>> = BinaryHeap::new();

        let mut head = ListNode { val: 0, next: None };
        let mut tail = &mut head;

        for entry in lists {
            if let Some(node) = entry {
                heap.push(Reverse(Wrapper(node)))
            }
        }

        while let Some(Reverse(Wrapper(mut top))) = heap.pop() {
            if let Some(next) = top.next.take() {
                heap.push(Reverse(Wrapper(next)))
            }

            tail.next = Some(top);
            tail = tail.next.as_mut().unwrap();
        }

        head.next
    }
}
