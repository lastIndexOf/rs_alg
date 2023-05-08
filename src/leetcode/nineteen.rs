#![allow(unused)]

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

pub struct Solution;

impl Solution {
    pub fn remove_nth_from_end(head: &mut Box<ListNode>, n: i32) -> Option<&Box<ListNode>> {
        let mut idx = 0;
        let mut node = &mut head.clone();
        let mut parent = None;

        while let Some(node_box) = node.next.as_mut() {
            node = node_box;
            idx += 1;

            if idx == n {
                if parent.is_none() {
                    parent = Some(head.clone());
                } else {
                    parent = parent.unwrap().next;
                }
            }
        }

        if parent.is_none() {
            return head.next.as_ref();
        }

        let cloned = parent.as_ref().unwrap().next.as_ref().unwrap().next.clone();
        parent.unwrap().next = cloned;

        Some(head)
    }
}

#[cfg(test)]
mod nineteen_test {
    // use super::*;

    #[test]
    fn test_nineteen() {}
}
