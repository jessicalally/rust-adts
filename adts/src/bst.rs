#[derive(Debug)]

pub enum Tree {
  Nil,  
  Leaf(u32),
  Node(u32, Box<Tree>, Box<Tree>),
}

impl Tree {
    pub fn new() -> Self {
        Tree::Nil
    }

    pub fn insert(self, val : u32) -> Tree {
        match self {
            Tree::Nil => Tree::Leaf(val),
           
            Tree::Leaf(other) => {
                if val > other {
                    Tree::Node(val, Box::new(Tree::Leaf(other)), Box::new(Tree::Nil)) 
                } else if val < other {
                    Tree::Node(val, Box::new(Tree::Nil), Box::new(Tree::Leaf(other)))        
                } else {
                    Tree::Leaf(other)
                }
            }
            
            Tree::Node(other, left, right) => {
                if val < other {
                    Tree::Node(other, Box::new(left.insert(val)), right)
                } else if val > other {
                    Tree::Node(other, left, Box::new(right.insert(val)))
                } else {
                    Tree::Node(other, left, right)
                }
            }
        }
    }

    pub fn find(&self, val : u32) -> bool {
        match self {
            Tree::Nil => false, 
            Tree::Leaf(other) => val == *other,
            Tree::Node(other, left, right) => {
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


