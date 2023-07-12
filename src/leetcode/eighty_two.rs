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
        let mut root = Some(root);

        let mut current = &mut root;
        let mut prev_val = None;
        let mut repeat = false;

        while let Some(current_node) = current.as_ref() {
            if let Some(next_node) = current_node.next.as_ref() {
                if let Some(val) = prev_val {
                    if val == next_node.val {
                        repeat = true;
                        let next = current.as_mut().unwrap().next.as_mut().unwrap().next.take();
                        current.as_mut().unwrap().next = next;
                    } else {
                        if val == current_node.val && repeat {
                            let next = current.as_mut().unwrap().next.take();
                            *current = next;
                            prev_val = Some(current.as_ref().unwrap().val);
                        } else {
                            prev_val = Some(next_node.val);
                            current = &mut current.as_mut().unwrap().next;
                        }
                        repeat = false;
                    }
                } else {
                    prev_val = Some(next_node.val);
                    current = &mut current.as_mut().unwrap().next;
                }
            } else {
                if let Some(val) = prev_val {
                    if repeat && val == current_node.val {
                        *current = None;
                    }
                }

                break;
            }
        }

        root.unwrap().next
    }
}

#[cfg(test)]
mod eighty_two_test {
    // use super::*;

    #[test]
    fn test_eighty_two() {}
}
