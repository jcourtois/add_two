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
    fn new(val: i32, next: Option<Box<ListNode>>) -> Box<ListNode> {
        Box::new(ListNode {
            val,
            next: next,
        })
    }

    fn from_array(input: &[i32]) -> Option<Box<ListNode>> {
        let n = if input.len() > 1 {
            Self::from_array(&input[1..])
        } else {
            None
        };
        Some(ListNode::new(input[0], n))
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add_two_numbers_rec(l1, l2, 0)
}

fn add_two_numbers_rec(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if let Some((a, b)) = l1.zip(l2) {
        let sum = a.val + b.val + carry;
        let carry_next = if sum < 10 { 0 } else { 1 };
        Some(Box::new(ListNode {
            val: sum % 10,
            next: add_two_numbers_rec(a.next, b.next, carry_next),
        }))
    } else {
        None
    }
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
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_array(&[]))
    }

    #[test]
    fn case3() {
        let l1 = ListNode::from_array(&[9,9,9,9,9,9,9]);
        let l2 = ListNode::from_array(&[9,9,9,9]);
        assert_eq!(add_two_numbers(l1, l2), ListNode::from_array(&[8,9,9,9,0,0,0,1]))
    }
}
