// Implementation of a 2-3 Tree.
//
// See https://en.wikipedia.org/wiki/2%E2%80%933_tree
// This implementation uses recursion to traverse down and up the tree, thus avoid
// having a parent pointer in the node. This also helps to conform to the borrow checker.

use std::cmp::Ordering;

// For simplicity, assume an Element has a usize key and value.
// This can be parameterized.
#[derive(Clone, Copy)]
pub struct Element {
    pub key: usize,
    pub value: usize,
}

impl std::cmp::PartialEq for Element {
    fn eq(&self, other: &Element) -> bool {
        self.key == other.key
    }
}

impl std::cmp::PartialOrd for Element {
    fn partial_cmp(&self, other: &Element) -> Option<std::cmp::Ordering> {
        self.key.partial_cmp(&other.key)
    }
}

// A node in the tere. No parent pointer here.
struct TwoThreeNode {
    elem1: Element,
    elem2: Option<Element>,
    child1: Option<Box<TwoThreeNode>>,
    child2: Option<Box<TwoThreeNode>>,
    child3: Option<Box<TwoThreeNode>>,
}

// A 2-3 Tree.
pub struct TwoThreeTree {
    root: Option<Box<TwoThreeNode>>,

    // Number of elements in the tree.
    size: usize,
}

// Used in Insertion phase.
struct InsertSubtree {
    parent_element: Element,
    child1: Box<TwoThreeNode>,
    child2: Box<TwoThreeNode>,
}

// Tracks the phase of the deletion operation.
enum DeletePhase {
    // Traversing downwards.
    Downwards,

    // To fix a hole in the tree by mutating the elements and branches.
    FixHole,

    // Done, true if the element was found and deleted.
    Done(bool),
}

// Tracks the state of the delete operation.
struct DeleteState {
    // The deletion element key.
    key: usize,

    // The current phase of the operation.
    phase: DeletePhase,

    // The predecessor of the element to be deleted.
    predecessor: Option<Element>,
}

impl TwoThreeTree {
    pub fn new() -> TwoThreeTree {
        TwoThreeTree {
            root: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    // Prints a textual representation of the tree.
    pub fn print(&self) {
        if let Some(ref root_node) = self.root {
            println!("Tree({}):", self.size);
            Self::print_node(root_node, 0);
        } else {
            println!("Empty tree");
        }
    }

    // Prints a node recursively.
    fn print_node(node: &TwoThreeNode, indent: usize) {
        for _ in 0..indent {
            print!("| ");
        }
        print!("Element: {}", node.elem1.key);
        if let Some(elem2) = node.elem2 {
            print!(" {}", elem2.key);
        }
        println!();
        if let Some(ref child1) = node.child1 {
            Self::print_node(child1, indent + 1);
        }
        if let Some(ref child2) = node.child2 {
            Self::print_node(child2, indent + 1);
        }
        if let Some(ref child3) = node.child3 {
            Self::print_node(child3, indent + 1);
        }
    }

    // Inserts an element.
    pub fn insert(&mut self, element: Element) {
        match &mut self.root {
            None => {
                self.root = Some(Self::new_node(element));
            }
            Some(ref mut root_node) => {
                if let Some(new_subtree) = Self::insert_node(root_node.as_mut(), &element) {
                    let mut new_root = Self::new_node(new_subtree.parent_element);
                    new_root.child1 = Some(new_subtree.child1);
                    new_root.child2 = Some(new_subtree.child2);
                    self.root = Some(new_root);
                }
            }
        }
        self.size += 1;
    }

    // Inserts a node, recursively.
    fn insert_node(node: &mut TwoThreeNode, element: &Element) -> Option<InsertSubtree> {
        if let Some(ref mut child) = node.child1 {
            // Not a leaf node.
            if element.key <= node.elem1.key {
                // Insert element in child1 subtree.
                let result = Self::insert_node(child, element);
                if let Some(new_subtree) = result {
                    match node.elem2 {
                        None => {
                            //    (a)           (result.parent_element, a)
                            //  /    \      =>    /           |           \
                            // result (b)     result.child1 result.child2 (b)
                            node.elem2 = Some(node.elem1);
                            node.elem1 = new_subtree.parent_element;
                            node.child3 = node.child2.take();
                            node.child1 = Some(new_subtree.child1);
                            node.child2 = Some(new_subtree.child2);
                            return None;
                        }
                        Some(elem2) => {
                            //      (a,b)                         (a)
                            //    /    |  \     =>             /       \
                            // result (c) (d)      result.parent         (b)
                            //                        /      \            /  \
                            //               result.child1 result.child2 (c) (d)
                            let mut left_node = Self::new_node(new_subtree.parent_element);
                            left_node.child1 = Some(new_subtree.child1);
                            left_node.child2 = Some(new_subtree.child2);

                            let mut right_node = Self::new_node(elem2);
                            right_node.child1 = node.child2.take();
                            right_node.child2 = node.child3.take();

                            return Some(InsertSubtree {
                                parent_element: node.elem1,
                                child1: left_node,
                                child2: right_node,
                            });
                        }
                    }
                } else {
                    return None;
                }
            }

            if node.elem2.is_none() || element.key <= node.elem2.unwrap().key {
                // Insert element under child2 subtree.
                let result = Self::insert_node(node.child2.as_mut().unwrap(), element);
                if let Some(new_subtree) = result {
                    match node.elem2 {
                        None => {
                            //   (a)           (a, result.parent_element)
                            //  /   \      =>    /     |         \
                            // (b) result       b, result.child1 result.child2
                            node.elem2 = Some(new_subtree.parent_element);
                            node.child2 = Some(new_subtree.child1);
                            node.child3 = Some(new_subtree.child2);
                            return None;
                        }
                        Some(elem2) => {
                            //     (a, b)                 result.parent_element
                            //   /   |    \      =>   (a)                       (b)
                            //  (c) result (d)       /  \                     /   \
                            //                      (c) result.child1  result.child2 (d)
                            let mut left_node = Self::new_node(node.elem1);
                            left_node.child1 = node.child1.take();
                            left_node.child2 = Some(new_subtree.child1);
                            let mut right_node = Self::new_node(elem2);
                            right_node.child1 = Some(new_subtree.child2);
                            right_node.child2 = node.child3.take();
                            return Some(InsertSubtree {
                                parent_element: new_subtree.parent_element,
                                child1: left_node,
                                child2: right_node,
                            });
                        }
                    }
                } else {
                    return None;
                }
            }

            // Insert element under child3 subtree.
            let result = Self::insert_node(node.child3.as_mut().unwrap(), element);
            if let Some(new_subtree) = result {
                //    (a,b)                     (b)
                //   /  |  \           =>     /     \
                //  (c) (d) result           (a)     (result.parent)
                //                          /  \      /             \
                //                         (c) (d) result.child1 result.child2
                let mut left_node = Self::new_node(node.elem1);
                left_node.child1 = node.child1.take();
                left_node.child2 = node.child2.take();
                let mut right_node = Self::new_node(new_subtree.parent_element);
                right_node.child1 = Some(new_subtree.child1);
                right_node.child2 = Some(new_subtree.child2);
                return Some(InsertSubtree {
                    parent_element: node.elem2.unwrap(),
                    child1: left_node,
                    child2: right_node,
                });
            } else {
                return None;
            }
        }

        // Handle leaf node.
        if let Some(elem2) = node.elem2 {
            if element.key < node.elem1.key {
                return Some(InsertSubtree {
                    parent_element: node.elem1,
                    child1: Self::new_node(*element),
                    child2: Self::new_node(elem2),
                });
            }
            if element.key < elem2.key {
                return Some(InsertSubtree {
                    parent_element: *element,
                    child1: Self::new_node(node.elem1),
                    child2: Self::new_node(elem2),
                });
            }
            return Some(InsertSubtree {
                parent_element: elem2,
                child1: Self::new_node(node.elem1),
                child2: Self::new_node(*element),
            });
        }
        if node.elem1.key <= element.key {
            node.elem2 = Some(*element);
        } else {
            node.elem2 = Some(node.elem1);
            node.elem1 = *element;
        }
        None
    }

    // Deletes an element with the given key.
    // Returns true if the element is found and deleted.
    pub fn delete(&mut self, key: usize) -> bool {
        let mut state = DeleteState {
            key,
            phase: DeletePhase::Downwards,
            predecessor: None,
        };

        if let Some(ref mut root) = self.root {
            Self::delete_node(root, &mut state);
            match state.phase {
                DeletePhase::Done(success) => {
                    if success {
                        self.size -= 1;
                    }
                    success
                }
                DeletePhase::FixHole => {
                    self.root = root.child1.take();
                    self.size -= 1;
                    true
                }
                DeletePhase::Downwards => panic!(),
            }
        } else {
            false
        }
    }

    // Deletes node recursively.
    fn delete_node(node: &mut TwoThreeNode, state: &mut DeleteState) {
        let child_num: u8;
        match node.child1 {
            // This is a leaf.
            None => {
                if node.elem1.key == state.key {
                    if let Some(elem2) = node.elem2 {
                        // Just move elem2 to elem1.
                        node.elem1 = elem2;
                        node.elem2 = None;
                        state.phase = DeletePhase::Done(true);
                        return;
                    }
                    // Leaf node is to be deleted.
                    state.phase = DeletePhase::FixHole;
                    return;
                }
                if let Some(elem2) = node.elem2 {
                    if elem2.key == state.key {
                        node.elem2 = None;
                        state.phase = DeletePhase::Done(true);
                        return;
                    }
                }
                // Not found.
                state.phase = DeletePhase::Done(false);
                return;
            }

            // Not leaf. Recursively go down the tree.
            Some(ref mut child1) => {
                match state.key.cmp(&node.elem1.key) {
                    Ordering::Less => {
                        Self::delete_node(child1, state);
                        child_num = 1;
                    }
                    Ordering::Greater => {
                        if let Some(elem2) = node.elem2 {
                            match state.key.cmp(&elem2.key) {
                                Ordering::Less => {
                                    Self::delete_node(node.child2.as_mut().unwrap(), state);
                                    child_num = 2;
                                }
                                Ordering::Greater => {
                                    Self::delete_node(node.child3.as_mut().unwrap(), state);
                                    child_num = 3;
                                }
                                Ordering::Equal => {
                                    // Matched. Find successor node.
                                    Self::find_predecessor(node.child2.as_mut().unwrap(), state);
                                    node.elem2 = Some(state.predecessor.unwrap());
                                    child_num = 2;
                                }
                            };
                        } else {
                            Self::delete_node(node.child2.as_mut().unwrap(), state);
                            child_num = 2;
                        }
                    }
                    Ordering::Equal => {
                        // Matched. Find succcessor node.
                        Self::find_predecessor(child1, state);
                        node.elem1 = state.predecessor.unwrap();
                        child_num = 1;
                    }
                }
            }
        }
        Self::delete_node_upward(node, child_num, state);
    }

    // Upward phase of the node deletion operation.
    fn delete_node_upward(node: &mut TwoThreeNode, child_num: u8, state: &mut DeleteState) {
        // Handle upward traversal.
        match state.phase {
            DeletePhase::Done(_) => (),

            // Fix a hole in the child by mutating the tree.
            DeletePhase::FixHole => {
                let child1 = node.child1.as_mut().unwrap();
                let child2 = node.child2.as_mut().unwrap();

                // If node is a 2-node.
                if node.elem2.is_none() {
                    if child_num == 1 {
                        // If Other child is a 2-node.
                        if child2.elem2.is_none() {
                            //   (a)              (o)
                            //  /   \      =>      |
                            // (o)  (b)           (a,b)
                            //  |   / \          /  |  \
                            // (c) (d) (e)      (c) (d) (e)
                            Self::add_left(child2, node.elem1, child1.child1.take());
                            node.child1 = node.child2.take();
                        } else {
                            //   (a)                 (b)
                            //  /   \      =>      /    \
                            // (o)  (b,c)        (a)    (c)
                            //  |   / | \        / \    / \
                            // (d) (e)(f)(g)   (d) (e) (f)(g)
                            child1.elem1 = node.elem1;
                            (node.elem1, child1.child2) = Self::trim_left(child2);
                            state.phase = DeletePhase::Done(true);
                        }
                    } else {
                        // If Other child is a 2-node.
                        if child1.elem2.is_none() {
                            //    (a)                (o)
                            //   /   \       =>       |
                            // (b)   (o)            (b,a)
                            // /  \   |            /  |  \
                            // ..    (c)           ..    (c)
                            Self::add_right(child1, node.elem1, child2.child1.take());
                        } else {
                            //      (a)               (c)
                            //    /     \      =>    /   \
                            //  (b,c)   (o)        (b)   (a)
                            //  / | \    |        / \    /  \
                            // (d)(e)(f) (g)    (d) (e) (f) (g)
                            child2.elem1 = node.elem1;
                            child2.child2 = child2.child1.take();
                            (node.elem1, child2.child1) = Self::trim_right(child1);
                            state.phase = DeletePhase::Done(true);
                        }
                    }
                    return;
                }

                // Node is a 3-node.
                let child3 = node.child3.as_mut().unwrap();
                if child_num == 1 {
                    // child2 is a 2-node.
                    if child2.elem2.is_none() {
                        //       (a,b)                   (b)
                        //     /   |   \                /   \
                        //   (o)  (c)  ..   =>       (a,c)   ..
                        //    |   / \                /  | \
                        //  (d) (e) (f)             (d)(e)(f)
                        Self::add_left(child2, node.elem1, child1.child1.take());
                        Self::trim_left(node);
                    } else {
                        //       (a,b)                    (c,b)
                        //     /   |   \                /   |   \
                        //   (o)  (c,d)  ..   =>       (a)  (d)  ..
                        //    |   / | \                / \   / \
                        //   (d) (e)(f)(g)            (d)(e)(f)(g)
                        child1.elem1 = node.elem1;
                        (node.elem1, child1.child2) = Self::trim_left(child2);
                    }
                } else if child_num == 2 {
                    if child1.elem2.is_none() {
                        //       (a,b)                   (b)
                        //     /    |   \               /   \
                        //   (c)   (o)  ..   =>      (c,a)  ..
                        //   / \    |                / \
                        //  (d)(e) (f)            (d)(e)(f)
                        Self::add_right(child1, node.elem1, child2.child1.take());
                        node.elem1 = node.elem2.take().unwrap();
                        node.child2 = node.child3.take();
                    } else {
                        //      (a,b)                   (d,b)
                        //     /  |   \               /   |   \
                        // (c,d)  (o)  ..   =>      (c)  (a)   ..
                        // / | \   |                / \  /  \
                        // ..  (e) (f)              ..  (e) (f)
                        child2.elem1 = node.elem1;
                        child2.child2 = child2.child1.take();
                        (node.elem1, child2.child1) = Self::trim_right(child1);
                    }
                } else if child2.elem2.is_none() {
                    //    (a,b)                  (a)
                    //   /  |   \               /   \
                    //  ..  (c)  (o)   =>      ..  (c,b)
                    //      / \   |                / | \
                    //    .. (d) (e)              .. (d)(e)
                    Self::add_right(child2, node.elem2.take().unwrap(), child3.child1.take());
                    node.child3 = None;
                } else {
                    //     (a,b)                  (a,d)
                    //   /   |   \               /  |   \
                    //  .. (c,d) (o)   =>      ..  (c)  (b)
                    //     / | \   |               / \  / \
                    //      .. (e) (f)            ..   (e)(f)
                    child3.elem1 = node.elem2.unwrap();
                    child3.child2 = child3.child1.take();
                    let result = Self::trim_right(child2);
                    node.elem2 = Some(result.0);
                    child3.child1 = result.1;
                }

                // Done.
                state.phase = DeletePhase::Done(true);
            }
            DeletePhase::Downwards => panic!(),
        }
    }

    // Finds an element with the given key.
    pub fn find(&self, key: usize) -> Option<Element> {
        if let Some(ref root) = self.root {
            let mut node = root;
            loop {
                match key.cmp(&node.elem1.key) {
                    Ordering::Less => {
                        if let Some(ref child1) = node.child1 {
                            node = child1;
                        } else {
                            return None;
                        }
                    }
                    Ordering::Greater => {
                        if let Some(elem2) = node.elem2 {
                            match key.cmp(&elem2.key) {
                                Ordering::Less => {
                                    if let Some(ref child2) = node.child2 {
                                        node = child2;
                                    } else {
                                        return None;
                                    }
                                }
                                Ordering::Greater => {
                                    if let Some(ref child3) = node.child3 {
                                        node = child3;
                                    } else {
                                        return None;
                                    }
                                }
                                Ordering::Equal => return Some(elem2),
                            }
                        } else if let Some(ref child2) = node.child2 {
                            node = child2;
                        } else {
                            return None;
                        }
                    }
                    Ordering::Equal => {
                        return Some(node.elem1);
                    }
                }
            }
        }
        None
    }

    // Converts a 2-node to a 3-node, adding a node and child on the left side.
    fn add_left(node: &mut TwoThreeNode, elem1: Element, child1: Option<Box<TwoThreeNode>>) {
        node.elem2 = Some(node.elem1);
        node.elem1 = elem1;
        node.child3 = node.child2.take();
        node.child2 = node.child1.take();
        node.child1 = child1;
    }

    // Converts a 2-node to a 3-node, adding a node and child on the right side.
    fn add_right(node: &mut TwoThreeNode, elem2: Element, child3: Option<Box<TwoThreeNode>>) {
        node.elem2 = Some(elem2);
        node.child3 = child3;
    }

    // Converts a 3-node to a 2-node, removing right element and right child.
    fn trim_right(node: &mut TwoThreeNode) -> (Element, Option<Box<TwoThreeNode>>) {
        (node.elem2.take().unwrap(), node.child3.take())
    }

    // Converts a 3-node to a 2-node, removing left element and left child.
    fn trim_left(node: &mut TwoThreeNode) -> (Element, Option<Box<TwoThreeNode>>) {
        let result = (node.elem1, node.child1.take());
        node.elem1 = node.elem2.take().unwrap();
        node.child1 = node.child2.take();
        node.child2 = node.child3.take();
        result
    }

    // Walk down the tree to the predecessor of a node.
    fn find_predecessor(node: &mut TwoThreeNode, state: &mut DeleteState) {
        if let Some(ref mut child3) = node.child3 {
            Self::find_predecessor(child3, state);
            Self::delete_node_upward(node, 3, state);
        } else if let Some(ref mut child2) = node.child2 {
            Self::find_predecessor(child2, state);
            Self::delete_node_upward(node, 2, state);
        } else {
            // Reached leaf node. Save the predecessor element.
            if node.elem2.is_some() {
                state.predecessor = node.elem2.take();
                state.phase = DeletePhase::Done(true);
            } else {
                state.predecessor = Some(node.elem1);
                state.phase = DeletePhase::FixHole;
            }
        }
    }

    // Creates a new node.
    fn new_node(element: Element) -> Box<TwoThreeNode> {
        Box::new(TwoThreeNode {
            elem1: element,
            elem2: None,
            child1: None,
            child2: None,
            child3: None,
        })
    }

    // Validates the structure of the tree.
    pub fn validate(&self) {
        if let Some(ref root) = self.root {
            let mut state = ValidateState::new();
            Self::validate_node(root, 0, &mut state);
            assert!(state.elements == self.size);
        }
    }

    // Validates a node recursively.
    fn validate_node(node: &TwoThreeNode, level: usize, state: &mut ValidateState) {
        state.elements += 1;

        // Check that elems are ordered.
        if let Some(elem2) = node.elem2 {
            assert!(node.elem1.key <= elem2.key);
            state.elements += 1;
        }

        // For leaf node.
        if node.child1.is_none() {
            assert!(node.child2.is_none());
            assert!(node.child3.is_none());

            // All leaves should be at the same level.
            if state.leaf_level == 0 {
                state.leaf_level = level;
            } else {
                assert!(level == state.leaf_level);
            }
            return;
        }

        // There should be at least 2 children.
        let child1 = node.child1.as_ref().unwrap();
        let child2 = node.child2.as_ref().unwrap();

        // Check child1, child2 ordering.
        Self::validate_node_less_than(child1, node.elem1.key);
        Self::validate_node_greater_than(child2, node.elem1.key);

        if let Some(elem2) = node.elem2 {
            // Check child3 ordering.
            let child3 = node.child3.as_ref().unwrap();
            Self::validate_node_greater_than(child3, elem2.key);
        }

        // Check the children.
        Self::validate_node(child1, level + 1, state);
        Self::validate_node(child2, level + 1, state);
        if let Some(ref child3) = node.child3 {
            Self::validate_node(child3, level + 1, state);
        }
    }

    // Checks that the node's elements are less than the given value.
    fn validate_node_less_than(node: &TwoThreeNode, key_value: usize) {
        assert!(node.elem1.key <= key_value);
        if let Some(elem2) = node.elem2 {
            assert!(elem2.key <= key_value);
        }
    }

    // Checks that the node's elements are greater than the given value.
    fn validate_node_greater_than(node: &TwoThreeNode, key_value: usize) {
        assert!(node.elem1.key >= key_value);
        if let Some(elem2) = node.elem2 {
            assert!(elem2.key >= key_value);
        }
    }
}

// Tracks the leaf level observed during validation recursion.
struct ValidateState {
    leaf_level: usize,
    elements: usize,
}

impl ValidateState {
    fn new() -> ValidateState {
        ValidateState {
            leaf_level: 0,
            elements: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Element, TwoThreeTree};

    fn insert(tree: &mut TwoThreeTree, key: usize) {
        println!("== Insert {}", key);
        tree.insert(Element {
            key: key,
            value: key,
        });
        tree.print();
        tree.validate();

        let found_element = tree.find(key);
        assert!(found_element.unwrap().key == key);
    }

    fn delete(tree: &mut TwoThreeTree, key: usize) {
        println!("== Delete {}", key);
        assert!(tree.delete(key));
        tree.print();
        tree.validate();
    }

    #[test]
    fn test_simple_1() {
        let mut tree = TwoThreeTree::new();
        insert(&mut tree, 2);
        insert(&mut tree, 1);
        insert(&mut tree, 3);
        insert(&mut tree, 5);
        insert(&mut tree, 4);
        assert!(tree.size() == 5);
        delete(&mut tree, 3);
        assert!(tree.find(3).is_none());
        delete(&mut tree, 1);
        delete(&mut tree, 2);
        delete(&mut tree, 4);
        delete(&mut tree, 5);
    }

    #[test]
    fn test_ordered_insert_delete() {
        let num_elements = 50;

        let mut tree = TwoThreeTree::new();
        for i in 0..num_elements {
            insert(&mut tree, i);
        }
        for i in 0..num_elements {
            delete(&mut tree, i);
        }
        assert!(tree.is_empty());

        for i in (0..num_elements).rev() {
            insert(&mut tree, i);
        }
        for i in 0..num_elements {
            delete(&mut tree, i);
        }
    }

    #[test]
    fn test_random_insert_delete() {
        let num_elements = 80;

        let mut tree = TwoThreeTree::new();
        let mut elements: Vec<usize> = Vec::new();
        for i in 0..num_elements {
            let elem = (num_elements + i * 71329) & 0xfffffff;
            elements.push(elem);
            insert(&mut tree, elem);
        }
        let mut n = 0;
        for _ in 0..elements.len() {
            n = (n + 13) % elements.len();
            delete(&mut tree, elements[n]);
        }
        assert!(tree.is_empty());
    }
}
