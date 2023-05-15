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
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let k = k as usize;
        let mut root = Box::new(ListNode::new(Default::default()));
        root.next = head;
        let mut parent = &mut root;
        let mut lists = vec![];

        'a: loop {
            for _ in 0..k {
                match parent.next.take() {
                    Some(next) => {
                        lists.push(next);
                        parent = lists.last_mut().unwrap();
                    }
                    None => break 'a,
                }
            }
        }

        parent = &mut root;
        let reverse_count = lists.len() / k;
        let rest_idx = reverse_count * k;

        for idx in 0..reverse_count {
            for k_idx in (0..k).rev() {
                let node = std::mem::replace(
                    &mut lists[idx * k + k_idx],
                    Box::new(ListNode::new(Default::default())),
                );

                parent.next = Some(node);
                parent = parent.next.as_mut().unwrap();
            }
        }

        for idx in rest_idx..lists.len() {
            let node =
                std::mem::replace(&mut lists[idx], Box::new(ListNode::new(Default::default())));
            parent.next = Some(node);
            parent = parent.next.as_mut().unwrap();
        }

        root.next
    }
}

#[cfg(test)]
mod twenty_five_test {
    use super::*;

    #[test]
    fn test_twenty_five() {
        let mut linked_list1 = ListNode::new(1);
        let mut linked_list2 = ListNode::new(2);
        let mut linked_list3 = ListNode::new(3);
        let mut linked_list4 = ListNode::new(4);
        let linked_list5 = ListNode::new(5);

        linked_list4.next = Some(Box::new(linked_list5));
        linked_list3.next = Some(Box::new(linked_list4));
        linked_list2.next = Some(Box::new(linked_list3));
        linked_list1.next = Some(Box::new(linked_list2));

        assert_eq!(format!("{:?}", Solution::reverse_k_group(Some(Box::new(linked_list1)), 2)), "Some(ListNode { val: 2, next: Some(ListNode { val: 1, next: Some(ListNode { val: 4, next: Some(ListNode { val: 3, next: Some(ListNode { val: 5, next: None }) }) }) }) })");

        let mut linked_list1 = ListNode::new(1);
        let mut linked_list2 = ListNode::new(2);
        let mut linked_list3 = ListNode::new(3);
        let mut linked_list4 = ListNode::new(4);
        let linked_list5 = ListNode::new(5);

        linked_list4.next = Some(Box::new(linked_list5));
        linked_list3.next = Some(Box::new(linked_list4));
        linked_list2.next = Some(Box::new(linked_list3));
        linked_list1.next = Some(Box::new(linked_list2));

        assert_eq!(format!("{:?}", Solution::reverse_k_group(Some(Box::new(linked_list1)), 3)), "Some(ListNode { val: 3, next: Some(ListNode { val: 2, next: Some(ListNode { val: 1, next: Some(ListNode { val: 4, next: Some(ListNode { val: 5, next: None }) }) }) }) })");

        // let linked_list1 = ListNode::new(1);
        // assert_eq!(
        //     format!("{:?}", Solution::reverse_k_group(Some(Box::new(linked_list1)))),
        //     "Some(ListNode { val: 1, next: None })"
        // );
    }
}
