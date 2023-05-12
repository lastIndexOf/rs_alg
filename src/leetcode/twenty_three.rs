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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut res = Box::new(ListNode::new(Default::default()));

        merge_any_lists(lists, &mut res);

        res.next
    }
}

fn merge_any_lists(lists: Vec<Option<Box<ListNode>>>, head: &mut Box<ListNode>) {
    // TODO: need loop
    let mut current_node = head;
    let mut lists = lists;

    loop {
        lists = lists
            .into_iter()
            .filter(|list| list.is_some())
            .collect::<Vec<_>>();

        if lists.is_empty() {
            break;
        }

        let (idx, min) = lists
            .iter_mut()
            .enumerate()
            .min_by(|(_, p), (_, n)| {
                let p = p.as_ref().unwrap();
                let n = n.as_ref().unwrap();

                match (p.val, n.val) {
                    (p, n) if p < n => std::cmp::Ordering::Less,
                    _ => std::cmp::Ordering::Greater,
                }
            })
            .unwrap();

        let next = min.as_mut().unwrap().next.take();
        current_node.next = std::mem::replace(&mut lists[idx], next);
        current_node = current_node.next.as_mut().unwrap();
    }
}

#[cfg(test)]
mod twenty_three_test {
    use super::*;

    #[test]
    fn test_twenty_three() {}
}
