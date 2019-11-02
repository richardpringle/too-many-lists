use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T> where T: std::fmt::Debug {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
struct Node<T>{
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        let node = Node {
            elem,
            next: None,
            prev: None,
        };
        Rc::new(RefCell::new(node))
    }
}

impl<T> List<T> where T: std::fmt::Debug {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_node = Node::new(elem);

        match self.head.take() {
            Some(node) => {
                node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(node);
                self.head = Some(new_node);
            }
            None => {
                self.tail = Some(new_node.clone());
                self.head = Some(new_node);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);

        match self.tail.take() {
            Some(node) => {
                node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(node);
                self.tail = Some(new_node);
            }
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            match node.borrow_mut().next.take() {
                Some(next_node) => {
                    next_node.borrow_mut().prev.take();
                    self.head = Some(next_node);
                }
                None => {
                    self.tail.take();
                }
            };

            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|node| {
            match node.borrow_mut().prev.take() {
                Some(prev_node) => {
                    prev_node.borrow_mut().next.take();
                    self.tail = Some(prev_node);
                }
                None => {
                    self.head.take();
                }
            }

            Rc::try_unwrap(node).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<&T> {
        // self.head.as_ref().map(|node| {
        //     let elem = node.clone().borrow().elem;
        //     &elem
        // })
        unimplemented!()
    }

    pub fn peek_back() {
        unimplemented!()
    }

    pub fn peek_front_mut() {
        unimplemented!()
    }

    pub fn peek_back_mut() {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        list.push_back(6);
        list.push_back(7);
        list.push_back(8);

        assert_eq!(list.pop_front(), Some(6));
        assert_eq!(list.pop_back(), Some(8));
        assert_eq!(list.pop_back(), Some(7));
        assert_eq!(list.pop_back(), None);
    }
}
