use adts::linked_list::LinkedList;
use adts::generic_bst::BST;
use adts::bst::Tree;

fn main() {
    let tree = Tree::new();
    let tree = tree.insert(10);
    let tree = tree.insert(12);
    let tree = tree.insert(11);

    println!("{:?}", tree);
    println!("The tree contains 13: {}", tree.find(13));
    println!("The tree contains 12: {}", tree.find(12));

    let list = LinkedList::new();
    let list = list.prepend(10);
    let list = list.prepend(11);
    let list = list.append(12);

    println!("{:?}", list);
    println!("The list contains 13: {}", list.contains(13));
    println!("The list contains 12: {}", list.contains(12));

    let list = list.remove(12);
    
    println!("The list contains 12: {}", list.contains(12));

    let bst = BST::new();
    let bst = bst.insert('a');
    let bst = bst.insert('c');

    println!("{:?}", bst);
    println!("BST contains 'a': {}", bst.find('a'));
    println!("BST contains 'b': {}", bst.find('b'));

    let bst = bst.remove('a');
    println!("BST contains 'a': {}", bst.find('a'));
}
