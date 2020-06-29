#[derive(Debug)]

pub enum LinkedList<T> {
    Nil,
    Cons(T, Box<LinkedList<T>>),
}

impl <T : PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList::Nil
    }

    pub fn prepend(self, value : T) -> LinkedList<T> {
        LinkedList::Cons(value, Box::new(self))
    }

    pub fn append(self, value : T) -> LinkedList<T> {
        match self {
            LinkedList::Nil => LinkedList::Cons(value, Box::new(LinkedList::Nil)),
            LinkedList::Cons(item_val, tail) => LinkedList::Cons(item_val, Box::new(tail.append(value))),
        }
    }

    pub fn remove(self, value : T) -> LinkedList<T> {
        match self {
            LinkedList::Nil => LinkedList::Nil,
            LinkedList::Cons(item_val, tail) => {
                if item_val == value {
                    *tail
                } else {
                    LinkedList::Cons(item_val, Box::new(tail.remove(value)))
                }
            }
        }
    }

    pub fn contains(&self, value : T) -> bool {
        match self {
            LinkedList::Nil => false,
            LinkedList::Cons(other, tail) => {
                if *other == value {
                    true
                } else {
                    tail.contains(value)
                }
            }
        }
    }
}
