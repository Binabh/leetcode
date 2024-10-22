#[cfg(test)]
mod tests {
    use crate::singly_linked_list::ListNode;

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut new_head: Option<Box<ListNode>> = None;
        let mut head_copy = head.as_ref();
        while head_copy != None {
            if let Some(ref node) = head_copy {
                let new_node = ListNode {
                    val: node.val,
                    next: new_head,
                };
                new_head = Some(Box::new(new_node));
                head_copy = node.next.as_ref();
            }
        }
        new_head
    }

    #[test]
    fn test() {
        assert_eq!(
            reverse_list(Some(Box::new(ListNode { val: 1, next: None }))),
            Some(Box::new(ListNode { val: 1, next: None }))
        );
        assert_eq!(reverse_list(None), None);
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(1)))
                }))
            }))
        );
        assert_eq!(
            reverse_list(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2)))
            }))),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(1)))
            }))
        );
    }
}
