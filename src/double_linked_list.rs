use std::cell::RefCell;
use std::rc::*;

pub struct DoublyLinkedList<T> {
    prev: Option<Weak<RefCell<DoublyLinkedList<T>>>>,
    next: Option<Rc<RefCell<DoublyLinkedList<T>>>>,
    value: T,
}

impl<T: Clone> DoublyLinkedList<T> {
    pub fn from_slice(data: &[T]) -> Option<Rc<RefCell<Self>>> {
        if data.len() == 0 {
            None
        } else {
            let head = Rc::new(RefCell::new(DoublyLinkedList {
                prev: None,
                next: None,
                value: data[0].clone(),
            }));
            data.iter()
                .skip(1)
                .map(|x| {
                    Rc::new(RefCell::new(DoublyLinkedList {
                        prev: None,
                        next: None,
                        value: x.clone(),
                    }))
                })
                .fold(head.clone(), |prev, current| {
                    prev.borrow_mut().next = Some(current.clone());
                    current.borrow_mut().prev = Some(Rc::downgrade(&prev));
                    current
                });
            Some(head)
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.next {
            Some(ref n) => write!(f, "{:?}->{:?}", self.value, n.borrow()),
            None => write!(f, "{:?}", self.value),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.prev {
            Some(ref n) => write!(
                f,
                "{}->{}",
                self.value,
                n.upgrade().ok_or_else(|| std::fmt::Error {})?.borrow()
            ),
            None => write!(f, "{}", self.value),
        }
    }
}

pub fn tail<T>(node: Rc<RefCell<DoublyLinkedList<T>>>) -> Rc<RefCell<DoublyLinkedList<T>>> {
    match node.borrow().next.as_ref() {
        Some(n) => tail(n.clone()),
        None => node.clone(),
    }
}


pub fn excute() {
    let data =
        DoublyLinkedList::from_slice(&["foo", "bar", "baz", "foobar"]).unwrap();
    println!("{:?}", data.borrow());
    println!("{}", tail(data.clone()).borrow());
}