use std::ptr;

struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct IntoIter<T>(List<T>);
struct Iter<'a, T> { next: Option<&'a Node<T>> }
struct IterMut<'a, T> { next: Option<&'a mut Node<T>> }

impl<T> Node<T> {
    pub fn new(elem: T) -> Box<Self> {
        Box::new(Node { elem, next: None })
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut node = Node::new(elem);
        let tail_ptr: *mut _ = &mut *node;

        if self.tail.is_null() {
            self.head = Some(node);
        } else {
            unsafe { (*self.tail).next = Some(node) }
        }

        self.tail = tail_ptr;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let node = *node;
            self.head = node.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        assert_eq!(list.peek(), Some(&1));

        if let Some(x) = list.peek_mut() {
            *x = 3;
        }

        assert_eq!(list.peek(), Some(&3));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));

        let mut other_iter = list.iter();
        assert_eq!(other_iter.next(), Some(&1));
        assert_eq!(other_iter.next(), Some(&2));

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(other_iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&3));

        assert_eq!(iter.next(), None);
        assert_eq!(other_iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);

        let mut iter = list.iter_mut();

        let x = iter.next().unwrap();

        *x = 3;

        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), None);

        assert_eq!(list.peek(), Some(&3));
    }
}
