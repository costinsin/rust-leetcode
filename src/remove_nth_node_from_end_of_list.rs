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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut n = n;
    let mut head = head;

    fn helper(head: &mut Option<Box<ListNode>>, n: &mut i32) -> bool {
        let mut result;

        if let Some(current_node) = head {
            result = helper(&mut (*current_node).next, n);
        } else {
            return true;
        }

        if *n == 0 {
            let next_next = head
                .as_ref()
                .unwrap()
                .next
                .as_ref()
                .unwrap()
                .next
                .to_owned();
            head.as_mut().unwrap().next = next_next;

            result = false;
        }

        *n -= 1;

        result
    }

    if head.is_none() {
        return None;
    }

    let head_should_be_removed = helper(&mut head, &mut n);

    if head_should_be_removed {
        head.as_ref().unwrap().next.to_owned()
    } else {
        head
    }
}

fn main() {}
