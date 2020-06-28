#[derive(Debug)]

enum BST<T> {
  Nil,  
  Leaf(T),
  Node(T, Box<BST<T>>, Box<BST<T>>),
}

impl <T : PartialOrd> BST<T> {
    fn new() -> Self {
        BST::Nil
    }

    fn insert(self, val : T) -> BST<T> {
        match self {
            BST::Nil => BST::Leaf(val),
           
            BST::Leaf(other) => {
                if val > other {
                    BST::Node(val, Box::new(BST::Leaf(other)), Box::new(BST::Nil)) 
                } else if val < other {
                    BST::Node(val, Box::new(BST::Nil), Box::new(BST::Leaf(other)))        
                } else {
                    BST::Leaf(other)
                }
            }
            
            BST::Node(other, left, right) => {
                if val < other {
                    BST::Node(other, Box::new(left.insert(val)), right)
                } else if val > other {
                    BST::Node(other, left, Box::new(right.insert(val)))
                } else {
                    BST::Node(other, left, right)
                }
            }
        }
    }

    fn find(&self, val : T) -> bool {
        match self {
            BST::Nil => false, 
            BST::Leaf(other) => val == *other,
            BST::Node(other, left, right) => {
                if val < *other {
                    left.find(val)
                } else if val > *other {
                    right.find(val)
                } else {
                    true
                }
            }
        }
    }
}

fn main() {
    let bst = BST::new();
    let bst = bst.insert('a');
    let bst = bst.insert('c');

    println!("{:?}", bst);
    println!("BST contains 'a': {}", bst.find('a'));
    println!("BST contains 'b': {}", bst.find('b'));
}

