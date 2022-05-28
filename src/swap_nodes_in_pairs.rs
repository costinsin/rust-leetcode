#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.and_then(|mut curr_node| match curr_node.next {
        Some(mut next_node) => {
            curr_node.next = swap_pairs(next_node.next);
            next_node.next = Some(curr_node);

            Some(next_node)
        }
        None => Some(curr_node),
    })
}
