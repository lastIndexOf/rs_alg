pub struct Solution;

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

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(-1));
        root.next = head;

        let mut current = &mut root.next;
        let mut prev_val = None;

        while let Some(current_node) = current.as_ref() {
            if let Some(val) = prev_val {
                if val == current_node.val {
                    let next = current.as_mut().unwrap().next.take();
                    *current = next;
                } else {
                    prev_val = Some(current_node.val);
                    current = &mut current.as_mut().unwrap().next;
                }
            } else {
                prev_val = Some(current_node.val);
                current = &mut current.as_mut().unwrap().next;
            }
        }

        root.next
    }
}

#[cfg(test)]
mod eighty_three_test {
    // use super::*;

    #[test]
    fn test_eighty_three() {}
}
