mod two_three_tree;

use rand::Rng;
use two_three_tree::{Element, TwoThreeTree};

fn insert(tree: &mut TwoThreeTree, key: usize) {
    //println!("== Insert {}", key);
    tree.insert(Element { key, value: key });
    tree.validate();
    assert!(tree.find(key).unwrap().key == key);
}

fn delete(tree: &mut TwoThreeTree, key: usize) {
    //println!("== Delete {}", key);
    assert!(tree.delete(key));
    tree.validate();
}

fn random_insert_delete(rng: &mut rand::rngs::ThreadRng) {
    let num_elements = 10000;
    let repetitions = 1;

    for _ in 0..repetitions {
        let mut tree = TwoThreeTree::new();
        let mut elements: Vec<usize> = Vec::new();

        // Insert.
        for _ in 0..num_elements {
            let elem: usize = rng.gen::<usize>() % 10000000;
            elements.push(elem);
            insert(&mut tree, elem);
        }
        assert!(tree.size() == num_elements);
        tree.print();

        // Delete.
        for i in (1..num_elements + 1).rev() {
            let n = rng.gen::<usize>() % i;
            let element = elements[n];
            elements[n] = elements[i - 1];
            delete(&mut tree, element);
        }
        assert!(tree.is_empty());
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    random_insert_delete(&mut rng);
}
