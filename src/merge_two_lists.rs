#[cfg(test)]
mod tests {
    use crate::singly_linked_list::ListNode;

    fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (Some(l1), Some(l2)) => match l1.val <= l2.val {
                true => Some(Box::new(ListNode {
                    val: l1.val,
                    next: self::merge_two_lists(l1.next, Some(l2)),
                })),
                false => Some(Box::new(ListNode {
                    val: l2.val,
                    next: self::merge_two_lists(Some(l1), l2.next),
                })),
            },
        }
    }
    #[test]
    fn test() {
        assert_eq!(
            merge_two_lists(
                Some(Box::new(ListNode { val: 1, next: None })),
                Some(Box::new(ListNode { val: 1, next: None }))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 1, next: None }))
            }))
        );
        assert_eq!(merge_two_lists(None, None), None);
        assert_eq!(
            merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3)))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3)))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode { val: 3, next: None }))
                            }))
                        }))
                    }))
                }))
            }))
        );
        assert_eq!(
            merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(4)))
                })),
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4)))
                    }))
                }))
            }))
        );
    }
}
