#[warn(dead_code)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

pub struct List {
    size: u32,
    head: Option<Box<Node>>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}
impl List {
    pub fn new() -> List {
        List {
            size: 0,
            head: None,
        }
    }

    pub fn push(&mut self, v: i32) -> Option<u32> {
        let new_node = Box::new(Node {
            value: v,
            next: self.head.take(),
        });

        self.head = Some(new_node);

        self.size += 1;
        Some(self.size - 1)
    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&mut self) -> Option<i32> {
        if self.size == 0 {
            return None;
        }
        self.head.as_ref().map(|node| node.value)
    }

    pub fn reverse(&mut self) -> List {
        let mut l = List::new();
        if self.size == 0 {
            return l;
        }
        let mut curr_node = &self.head;
        while let Some(node) = curr_node {
            l.push(node.value);
            curr_node = &node.next;
        }
        l
    }
}

#[cfg(test)]
mod list_tests {
    use super::List;

    #[test]
    fn create_list_test() {
        let l = List::new();
        assert_eq!(l.size, 0);
    }

    #[test]
    fn push_test() {
        let mut l = List::new();
        let res = l.push(1);
        assert_eq!(l.size, 1);
        assert_eq!(res, Some(l.size - 1));
    }

    #[test]
    fn pop_test() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        assert_eq!(l.size, 3);
        assert_eq!(l.pop(), Some(3));
        assert_eq!(l.pop(), Some(2));
        assert_eq!(l.pop(), Some(1));
    }

    #[test]
    fn peek_test() {
        let mut l = List::new();
        l.push(11);
        assert_eq!(l.peek(), Some(11));
    }

    #[test]
    fn reverse_test() {
        let mut l = List::new();
        l.push(1);
        l.push(2);
        l.push(3);
        let mut rl = l.reverse();
        assert_eq!(rl.size, 3);
        assert_eq!(rl.pop(), Some(1));
        assert_eq!(rl.pop(), Some(2));
        assert_eq!(rl.pop(), Some(3));
    }
}
