#![allow(dead_code, unused_macros)]

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

macro_rules! list_node {
    // Match the pattern: `($val:expr $(, $next_val:expr)*)`
    // This captures one or more expressions passed to the macro as values for the ListNode.
    ($val:expr $(, $next_val:expr)*) => {
        {
            let mut head = Box::new(ListNode::new($val));
            let mut current_node = &mut head;
            $(
                let new_node = Box::new(ListNode::new($next_val));
                current_node.next = Some(new_node);
                current_node = current_node.next.as_mut().unwrap();
            )*
            current_node.next = None;
            head
        }
    };
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result: Box<ListNode> = Box::new(ListNode::new(0));
    let mut current_node = &mut result;
    let mut carry: i32 = 0;

    let mut l1_next = l1;
    let mut l2_next = l2;

    while l1_next.is_some() || l2_next.is_some() {
        let mut sum = carry;

        if l1_next.is_some() {
            let l1_current = l1_next.unwrap();
            sum = sum + l1_current.val;
            l1_next = l1_current.next;
        }
        if l2_next.is_some() {
            let l2_current = l2_next.unwrap();
            sum = sum + l2_current.val;
            l2_next = l2_current.next;
        }

        carry = sum / 10;
        current_node.val = sum % 10;

        if l1_next.is_some() || l2_next.is_some() {
            current_node.next = Some(Box::new(ListNode::new(0)));
            current_node = current_node.next.as_mut().unwrap();
        }
    }

    if carry > 0 {
        current_node.next = Some(Box::new(ListNode::new(carry)));
    } else {
        current_node.next = None;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let next = add_two_numbers(Some(list_node!(2, 4, 3)), Some(list_node!(5, 6, 4)));
        assert_eq!(next, Some(list_node!(7, 0, 8)));

        let next = add_two_numbers(
            Some(Box::new(ListNode::new(0))),
            Some(Box::new(ListNode::new(0))),
        );
        assert_eq!(next, Some(Box::new(ListNode::new(0))));

        let next = add_two_numbers(
            Some(list_node!(9, 9, 9, 9, 9, 9, 9)),
            Some(list_node!(9, 9, 9, 9)),
        );
        assert_eq!(next, Some(list_node!(8, 9, 9, 9, 0, 0, 0, 1)));
    }
}
