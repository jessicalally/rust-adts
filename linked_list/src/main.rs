#[derive(Debug)]

enum LinkedList<T> {
    Nil,
    Cons(T, Box<LinkedList<T>>),
}

impl <T> LinkedList<T> {
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
}

fn main() {
    let list = LinkedList::new();
    let list = list.prepend(10);
    let list = list.prepend(11);
    let list = list.append(12);

    println!("{:#?}", list);
}
