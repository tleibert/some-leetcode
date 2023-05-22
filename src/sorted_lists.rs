use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
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

        let mut head = ListNode::new(0);
        let mut tail = &mut head;

        for node in lists.into_iter().flatten() {
            heap.push(Reverse(Wrapper(node)))
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

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut h1 = l1;
        let mut h2 = l2;

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut carry = 0;

        while h1.is_some() || h2.is_some() {
            let n1 = h1.as_ref().map(|node| node.val).unwrap_or(0);
            let n2 = h2.as_ref().map(|node| node.val).unwrap_or(0);
            h1 = h1.and_then(|node| node.next);
            h2 = h2.and_then(|node| node.next);

            let next = Box::new(ListNode::new((n1 + n2 + carry) % 10));
            carry = (n1 + n2 + carry) / 10;
            tail.next = Some(next);
            tail = tail.next.as_mut().unwrap();
        }

        if carry != 0 {
            let new = Box::new(ListNode::new(carry));
            tail.next = Some(new);
        }

        dummy.next
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for item in nums {
            heap.push(item);
            if heap.len() > k as usize && item < *heap.peek().unwrap_or(&i32::MIN) {
                heap.pop();
            }
        }

        *heap.peek().unwrap()
    }

    fn reverse(node: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {
        let mut stack = Vec::with_capacity(k);
        let mut head = node;
        for _ in 0..k {
            if let Some(mut node) = head {
                head = node.next.take();
                stack.push(node);
            }
        }

        head
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        head
    }

    pub fn add_two_numbers_2(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut carry = 0;

        loop {
            match (l1.take(), l2.take()) {
                (Some(mut n1), Some(mut n2)) => {
                    l1 = n1.next.take();
                    l2 = n2.next.take();

                    // add with carry
                    let sum = n1.val + n2.val + carry;
                    carry = sum / 10;
                    n1.val = sum % 10;

                    // re-use n1 allocation
                    tail.next = Some(n1);
                    tail = tail.next.as_mut().unwrap();
                }
                (Some(mut n1), None) => {
                    l1 = n1.next.take();

                    // add with carry
                    let sum = n1.val + carry;
                    carry = sum / 10;
                    n1.val = sum % 10;

                    // re-use n1 allocation
                    tail.next = Some(n1);
                    tail = tail.next.as_mut().unwrap();
                }
                (None, Some(mut n2)) => {
                    l2 = n2.next.take();

                    // add with carry
                    let sum = n2.val + carry;
                    carry = sum / 10;
                    n2.val = sum % 10;

                    // re-use n2 allocation
                    tail.next = Some(n2);
                    tail = tail.next.as_mut().unwrap();
                }
                (None, None) => {
                    // add the carry digit if it's not 0
                    if carry > 0 {
                        tail.next = Some(Box::new(ListNode::new(carry)));
                    }
                    break;
                }
            };
        }

        dummy.next.take()
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.and_then(|mut head| match head.next.take() {
            // head.next is now None, and we own it
            Some(mut next) => {
                // get raw pointer to the box's contents
                let mut next_ptr: *mut ListNode = &mut *next;
                let node = Solution::reverse_list(Some(next));
                unsafe {
                    (*next_ptr).next = Some(head);
                }
                node
            }
            None => Some(head),
        })
    }

    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;

        while let Some(mut node) = head {
            head = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(mut node1), Some(mut node2)) => {
                if node1.val < node2.val {
                    node1.next = Self::merge_two_lists(node1.next, Some(node2));
                    Some(node1)
                } else {
                    node2.next = Self::merge_two_lists(Some(node1), node2.next);
                    Some(node2)
                }
            }
            (None, Some(node2)) => Some(node2),
            (Some(node1), None) => Some(node1),
            (None, None) => None,
        }
    }
}
