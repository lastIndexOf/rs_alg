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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(-1));
        root.next = head;
        let mut next = &mut root.next;
        let mut prev_val = None;

        while let Some(node) = next.as_mut() {
            if let Some(prev) = prev_val {
                if prev == node.val {
                    let next_node = node.next.take().unwrap();
                    *node = next_node;
                    prev_val = Some(node.val);
                    next = &mut node.next;
                } else {
                    prev_val = Some(node.val);
                    next = &mut node.next;
                }
            } else {
                prev_val = Some(node.val);
                next = &mut node.next;
            }
        }

        root.next
    }
}

#[cfg(test)]
mod eighty_two_test {
    // use super::*;

    #[test]
    fn test_eighty_two() {}
}
