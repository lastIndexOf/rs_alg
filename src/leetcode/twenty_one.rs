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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = list1;
        let mut l2 = list2;
        let mut res = Box::new(ListNode::new(Default::default()));
        let mut res_mut = &mut res;

        loop {
            match (l1.as_mut(), l2.as_mut()) {
                (Some(list1_node), Some(list2_node)) => {
                    if list1_node.val < list2_node.val {
                        let next = list1_node.next.take();
                        (*res_mut).next = l1;
                        l1 = next;
                    } else {
                        let next = list2_node.next.take();
                        (*res_mut).next = l2;
                        l2 = next;
                    }
                    res_mut = (*res_mut).next.as_mut().unwrap();
                }
                (Some(_), None) => {
                    (*res_mut).next = l1;
                    break;
                }
                (None, Some(_)) => {
                    (*res_mut).next = l2;
                    break;
                }
                (None, None) => break,
            }
        }

        res.next
    }
}

#[cfg(test)]
mod twenty_one_test {
    // use super::*;

    #[test]
    fn test_twenty_one() {}
}
