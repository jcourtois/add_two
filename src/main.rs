fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> ListNode {
        ListNode { next: None, val }
    }

    fn from_array(input: &[i32]) -> Option<Box<ListNode>> {
        match input.len() {
            0 => None,
            1 => Some(Box::new(ListNode { val: input[0], next: None })),
            _ => Some(Box::new(ListNode { val: input[0], next: Self::from_array(&input[1..]) }))
        }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut p = l1;
    let mut q = l2;

    let mut carry: i32 = 0;

    while p != None || q != None {
        let sum = match (&p, &q) {
            (Some(l1), Some(l2)) => l1.val + l2.val + carry,
            (Some(l1), None) => l1.val + carry,
            (None, Some(l2)) => l2.val + carry,
            (None, None) => carry,
        };

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        p = if p != None { p.unwrap().next } else { p };
        q = if q != None { q.unwrap().next } else { q };
    }

    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    dummy_head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let l1 = ListNode::from_array(&[2, 4, 3]);
        let l2 = ListNode::from_array(&[5, 6, 4]);
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_array(&[7, 0, 8]))
    }

    #[test]
    fn case2() {
        let l1 = ListNode::from_array(&[0]);
        let l2 = ListNode::from_array(&[0]);
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_array(&[0]))
    }

    #[test]
    fn case3() {
        let l1 = ListNode::from_array(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = ListNode::from_array(&[9, 9, 9, 9]);
        assert_eq!(
            add_two_numbers(l1, l2),
            ListNode::from_array(&[8, 9, 9, 9, 0, 0, 0, 1])
        )
    }
}
