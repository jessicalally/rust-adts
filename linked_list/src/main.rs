#[derive(Debug)]

enum LinkedList<T> {
    Nil,
    Cons(T, Box<LinkedList<T>>),
}

impl <T : PartialOrd> LinkedList<T> {
    fn new() -> Self {
        LinkedList::Nil
    }

    fn prepend(self, value : T) -> LinkedList<T> {
        LinkedList::Cons(value, Box::new(self))
    }

    fn append(self, value : T) -> LinkedList<T> {
        match self {
            LinkedList::Nil => LinkedList::Cons(value, Box::new(LinkedList::Nil)),
            LinkedList::Cons(item_val, tail) => LinkedList::Cons(item_val, Box::new(tail.append(value))),
        }
    }

    fn contains(&self, value : T) -> bool {
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

fn main() {
    let list = LinkedList::new();
    let list = list.prepend(10);
    let list = list.prepend(11);
    let list = list.append(12);

    println!("{:#?}", list);

    println!("The list contains 13: {}", list.contains(13));
    println!("The list contains 12: {}", list.contains(12));
}
