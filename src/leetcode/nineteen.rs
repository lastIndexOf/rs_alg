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
    // clone 相当于把链表的值深拷贝了一份，不是最优解
    // 下面的实现还是深拷贝了部分内存数据
    // 可以直接在链表中存储原始指针
    // pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    //     let mut dummy_head = Box::new(ListNode::new(0));
    //     dummy_head.next = head;
    //     let mut fast = &dummy_head.clone();
    //     let mut slow = &mut dummy_head;
    //     for i in 1..=n {
    //         fast = fast.next.as_ref().unwrap();
    //     }
    //     while fast.next.is_some() {
    //         fast = fast.next.as_ref().unwrap();
    //         slow = slow.next.as_mut().unwrap();
    //     }
    //     slow.next = slow.next.as_mut().unwrap().next.take();
    //     return dummy_head.next;
    // }

    pub fn remove_nth_from_end(head: &mut Box<ListNode>, n: i32) -> Option<&Box<ListNode>> {
        let mut faster = &mut **head as *mut ListNode;
        let mut slower = &mut **head as *mut ListNode;

        unsafe {
            for _ in 0..n {
                if (*faster).next.is_none() {
                    return head.next.as_ref();
                }
                faster = &mut **((*faster).next.as_mut().unwrap()) as *mut ListNode;
            }

            while !(*faster).next.is_none() {
                faster = &mut **((*faster).next.as_mut().unwrap()) as *mut ListNode;
                slower = &mut **((*slower).next.as_mut().unwrap()) as *mut ListNode;
            }

            (*slower).next = (*slower).next.as_ref().unwrap().next.clone();

            Some(head)
        }
    }
}

#[cfg(test)]
mod nineteen_test {
    use crate::data_struct::linked_list;

    use super::*;

    #[test]
    fn test_nineteen() {
        let mut linked_list = ListNode::new(1);
        linked_list.next = Some(Box::new(ListNode::new(2)));
        linked_list.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        linked_list
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(4)));
        linked_list
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next
            .as_mut()
            .unwrap()
            .next = Some(Box::new(ListNode::new(5)));

        let mut linked_list = &mut Box::new(linked_list);

        assert_eq!(format!("{:?}", Solution::remove_nth_from_end(&mut linked_list, 3)), "Some(ListNode { val: 1, next: Some(ListNode { val: 2, next: Some(ListNode { val: 4, next: Some(ListNode { val: 5, next: None }) }) }) })");

        println!("linked_list = {linked_list:?}");
        // assert_eq!(format!("{:?}", Solution::remove_nth_from_end(&mut linked_list, 1)), "Some(ListNode { val: 2, next: Some(ListNode { val: 3, next: Some(ListNode { val: 4, next: None }) }) })");
    }
}
