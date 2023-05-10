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
        let mut list1 = list1.clone();
        let mut list2 = list2.clone();
        let mut res = Box::new(ListNode::new(0));
        let mut res_mut = &mut res;

        loop {
            match (list1.as_mut(), list2.as_mut()) {
                (Some(list1_node), Some(list2_node)) => {
                    if list1_node.val < list2_node.val {
                        let node = list1_node.next.take();
                        res_mut.next = list1;
                        list1 = res_mut.next;
                    } else {
                        let node = list2_node.next.take();
                        res_mut.next = list2;
                        list2 = res_mut.next;
                    }
                }
                (Some(list1_node), None) => {
                    res_mut.next = list1_node.next.take();
                    res_mut = res_mut.next.as_mut().unwrap();
                    break;
                }
                (None, Some(list2_node)) => {
                    res_mut.next = list2_node.next.take();
                    res_mut = res_mut.next.as_mut().unwrap();
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
