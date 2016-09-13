use std::cmp::Ord;

enum Color {
    Red,
    Black,
}

struct Node<T: Ord> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    color: Color,
    data: T,
}

pub struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
    num_items: u64,
}

impl <T: Ord> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root: None,
            num_items: 0,
        }
    }

    pub fn insert(mut self, data: T) {
        if self.num_items == 0 {
            // case: tree is empty
            self.insert_empty(data);
        } else {
        }
    }

    pub fn remove(mut self, data: T) {
    }

    pub fn exists(self, data: T) -> bool {
        false
    }

    pub fn traverse(self) -> Vec<T> {
        Vec::new()
    }

    fn bst_insert(mut self, data: T) {
        let mut node = Some(self.root.unwrap());
        let mut parent;

        while node.is_some() {
            let less_than_parent;
            parent = node.unwrap();

            node = match data < parent.data {
                true => {
                    less_than_parent = true;
                    parent.left
                },
                false => {
                    less_than_parent = false;
                    parent.right
                },
            };

            if node.is_none() {
                let new_node = Some(Box::new(Node {
                    left: None,
                    right: None,
                    color: Color::Red,
                    data: data,
                }));

                if less_than_parent {
                    parent.left = new_node;
                } else {
                    parent.right = new_node;
                }
            }
        }



    }

//    /// Naive binary search tree leaf insertion
//    fn _bst_insert(mut self, data: T) {
//        let mut node = self.root;
//        let mut parent;
//        self.num_items += 1;
//
//        let new_node = Some(Box::new(Node {
//            left: None,
//            right: None,
//            color: Color::Red,
//            data: data,
//        }));
//
//        // find node onto which to add data as leaf
//        while node.is_some() {
//            let parent = &node;
//            let node_node = node.unwrap();
//            let node_data = &node_node.data;
//            let parent_node = node_node;
//
//            node = match &data < node_data {
//                true => node_node.left,
//                false => node_node.right,
//            };
//        }
//
//        if data < parent_node.data {
//            parent_node.left = new_node;
//        } else {
//            parent_node.right = new_node;
//        }
//    }

    /// Insertion for when the tree is empty
    fn insert_empty(mut self, data: T) {
        let node = Node {
            left: None,
            right: None,
            color: Color::Black,
            data: data,
        };
        self.root = Some(Box::new(node));
        self.num_items = 1;
    }

}
