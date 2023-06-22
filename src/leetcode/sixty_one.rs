pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(unused)]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut v = vec![];

        loop {
            match head.take() {
                Some(mut node) => {
                    head = node.next.take();
                    v.push(node);
                }
                None => break,
            }
        }

        if v.is_empty() {
            return None;
        }

        let idx = v.len() - 1;
        let mut k = k % v.len() as i32;
        while k > 0 {
            v.rotate_left(idx);
            k -= 1;
        }

        let mut end = match v.pop() {
            Some(node) => node,
            None => return None,
        };

        while let Some(mut node) = v.pop() {
            node.next = Some(end);
            end = node;
        }

        Some(end)
    }
}

#[cfg(test)]
mod sixty_one_test {
    // use super::*;

    #[test]
    fn test_sixty_one() {}
}
