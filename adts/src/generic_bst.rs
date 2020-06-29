#[derive(Debug)]

// boolean value indicates if value is still present in the tree or not

pub enum BST<T> {
  Nil,  
  Leaf(T, bool),
  Node(T, bool, Box<BST<T>>, Box<BST<T>>),
}

impl <T : PartialOrd> BST<T> {
    pub fn new() -> Self {
        BST::Nil
    }

    pub fn insert(self, val : T) -> BST<T> {
        match self {
            BST::Nil => BST::Leaf(val, true),
           
            BST::Leaf(other, is_valid) => {
                if val > other {
                    BST::Node(val, true, Box::new(BST::Leaf(other, is_valid)), Box::new(BST::Nil)) 
                } else if val < other {
                    BST::Node(val, true, Box::new(BST::Nil), Box::new(BST::Leaf(other, is_valid)))        
                } else {
                    BST::Leaf(other, true)
                }
            }
            
            BST::Node(other, is_valid, left, right) => {
                if val < other {
                    BST::Node(other, is_valid, Box::new(left.insert(val)), right)
                } else if val > other {
                    BST::Node(other, is_valid, left, Box::new(right.insert(val)))
                } else {
                    BST::Node(other, true, left, right)
                }
            }
        }
    }

    pub fn find(&self, val : T) -> bool {
        match self {
            BST::Nil => false, 
            BST::Leaf(other, is_valid) => (val == *other) && *is_valid,
            BST::Node(other, is_valid, left, right) => {
                if val < *other {
                    left.find(val)
                } else if val > *other {
                    right.find(val)
                } else {
                    *is_valid
                }
            }
        }
    }

    // lazy removal
    pub fn remove(self, val : T) -> BST<T> {
        match self {
            BST::Nil => BST::Nil,
            
            BST::Leaf(other, is_valid) => {
                if val == other {
                    BST::Leaf(other, false)
                } else {
                    BST::Leaf(other, is_valid)
                }
            }
            BST::Node(other, is_valid, left, right) => {
                if val == other {
                    BST::Node(other, false, left, right)
                } else if val < other {
                    BST::Node(other, is_valid, Box::new(left.remove(val)), right)
                } else {
                    BST::Node(other, is_valid, left, Box::new(right.remove(val)))
                }   
            }
        }
    }
}


