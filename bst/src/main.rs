#[derive(Debug)]

enum Tree {
  Nil,  
  Leaf(u32),
  Node(u32, Box<Tree>, Box<Tree>),
}

impl Tree {
    fn insert(self, val : u32) -> Tree {
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

    fn find(&self, val : u32) -> bool {
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


fn main() {
    let t = Tree::Nil;
    let t = t.insert(10);
    let t = t.insert(15);
    let t = t.insert(5);

    println!("{:?}", t);
    println!("Find {}: {}", 15, t.find(15));
    println!("Find {}: {}", 12, t.find(12));
}
