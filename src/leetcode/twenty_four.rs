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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = Box::new(ListNode::new(Default::default()));
        root.next = head;
        let mut parent = &mut root;

        loop {
            let mut left = parent.next.take();

            match left.as_mut() {
                Some(left_node) => {
                    let mut right = left_node.next.take();
                    match right.as_mut() {
                        Some(right_node) => {
                            let child = right_node.next.take();

                            left_node.next = child;
                            right_node.next = left;
                            parent.next = right;

                            if parent.next.is_none() {
                                break;
                            }

                            // 核心代码，所有权转移后要通过最终拥有其所有权的那个变量来获取可变引用
                            parent = parent.next.as_mut().unwrap().next.as_mut().unwrap();
                        }
                        None => {
                            left_node.next = right;
                            parent.next = left;
                            break;
                        }
                    }
                }
                None => {
                    parent.next = left;
                    break;
                }
            }
        }

        root.next
    }
}

#[cfg(test)]
mod twenty_four_test {
    use std::{assert_eq, format};

    use super::*;

    #[test]
    fn test_twenty_four() {
        let mut linked_list1 = ListNode::new(1);
        let mut linked_list2 = ListNode::new(2);
        let mut linked_list3 = ListNode::new(3);
        let linked_list4 = ListNode::new(4);

        linked_list3.next = Some(Box::new(linked_list4));
        linked_list2.next = Some(Box::new(linked_list3));
        linked_list1.next = Some(Box::new(linked_list2));

        assert_eq!(format!("{:?}", Solution::swap_pairs(Some(Box::new(linked_list1)))), "Some(ListNode { val: 2, next: Some(ListNode { val: 1, next: Some(ListNode { val: 4, next: Some(ListNode { val: 3, next: None }) }) }) })");

        let linked_list1 = ListNode::new(1);
        assert_eq!(
            format!("{:?}", Solution::swap_pairs(Some(Box::new(linked_list1)))),
            "Some(ListNode { val: 1, next: None })"
        );
    }
}
