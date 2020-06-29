use crate::linked_list::LinkedList;

// Implementation of a stack using the LinkedList ADT

#[derive(Debug)]
pub struct Stack<T> {
    list: LinkedList<T>,
    size: u32
}

impl <T : PartialOrd> Stack<T> {
    pub fn new() -> Self {
        Stack { list: LinkedList::new(), size: 0}
    }

    pub fn push(mut self, value : T) {
        self.list = self.list.prepend(value);
        self.size = self.size + 1;
    }

    pub fn pop(mut self) -> Result<T, String> {
        let (result, stack) = self.list.pop();
        match result {
            Ok(value) => {
                self.list = stack;
                self.size = self.size - 1;
                Ok(value)
            }

            Err(_) => Err(String::from("stack is empty"))
            
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}
