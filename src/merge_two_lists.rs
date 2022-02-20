use std::mem;

#[allow(dead_code, unused_variables)]
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

pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut p_next = &mut dummy;

    while l1.is_some() && l2.is_some() {
        let lone = &mut l1;
        let ltwo = &mut l2;
        let l = if lone.as_ref().unwrap().val < ltwo.as_ref().unwrap().val {
            lone
        } else {
            ltwo
        };

        mem::swap(p_next, l);
        mem::swap(l, &mut p_next.as_mut().unwrap().next);
        p_next = &mut p_next.as_mut().unwrap().next;
    }

    mem::swap(p_next, if l1.is_none() { &mut l2 } else { &mut l1 });
    dummy
}

fn main() {
    let list1 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    let list2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    }));

    merge_two_lists(list1, list2);
}
