/// Simple binary search tree
///
/// For every node of value `v`, all elements in the left sub-tree are smaller
/// than `v` and all elements in the right sub-tree are larger than `v`.
#[derive(Debug)]
pub struct Tree(Option<Box<Node>>);

/// Internal Node representation with a `value` and the left and right sub-trees.
#[derive(Debug)]
struct Node {
    value: i32,
    left: Tree,
    right: Tree,
}

impl Tree {
    /// Returns an empty tree
    pub fn new() -> Self {
        Tree(None)
    }

    pub fn insert(&mut self, val1: i32) -> bool {
        if self.value == val1 {
            false
        } else if val1 < self.value {
            match &mut self.left {
                Some(left) => left.insert(val1),
                None => {
                    self.left = Some(Box::new(Node::leaf(val1)));
                    true
                }
            }
        } else { 
            match &mut self.right {
                Some(right) => right.insert(val1),
                None => {
                    self.right = Some(Box::new(Node::leaf(val1)));
                    true
                }
            }
        }
    }

    /// Returns true if and only if `value` belongs to the tree.
    pub fn contains(&self, val2: i32) -> bool {
        if self.value == val2 {
            true
        } else {
            if self.value > val2 {
                match &self.left {
                    None => false,
                    Some(left) => left.contains(val2),
                }
            } else {
                match &self.right {
                    None => false,
                    Some(right) => right.contains(val2),
                }
            }
        }
    }

    /// Deletes `value` from the tree.
    /// When the value is not found the tree, `false` is returned.
    pub fn delete(&mut self, val: i32) {
    if self.value == val {
        self.value = 0;
        self.left = None;
        self.right = None;
    } else if val < self.value {
        match &mut self.left {
            Some(left) => {
                left.delete(val);
            }
            None => {}
        }
    } else {
        match &mut self.right {
            Some(right) => {
                right.delete(val);
            }
            None => {}
            }
        }
    }

}


fn main() {
    let t = Tree::new();
    println!("{:?}", t);
}

