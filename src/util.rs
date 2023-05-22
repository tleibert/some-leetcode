use meme::sorted_lists::ListNode;

pub fn build_list(mut num: i32) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut tail = &mut dummy;

    while num > 0 {
        let rem = num % 10;
        num /= 10;
        tail.next = Some(Box::new(ListNode::new(rem)));
        tail = tail.next.as_mut().unwrap();
    }

    dummy.next
}
