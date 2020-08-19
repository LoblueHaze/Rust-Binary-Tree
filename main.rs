/*
 *
 * Binary tree with pure rust. (rand used just for sample data.)
 * By Loblue Haze <loblue.haze@gmail.com>
 *
 */

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut tree = Tree::new();
    for _i in 0..10 {
        let random_number: i32 = rng.gen_range(-10, 10);
        tree = tree.add_value(random_number);
    }
    tree.print_tree();
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Self { root: None }
    }

    fn add_value(mut self, value: i32) -> Self {
        let node = Node::new(value);
        if self.root.is_none() {
            self.root = Some(Box::new(node));
            return self;
        } else {
            let tree = self.root.unwrap().add_node(node);
            return tree;
        }
    }

    fn tree_to_node(self) -> Node {
        *self.root.unwrap()
    }

    fn print_tree(self) {
        let root = self.tree_to_node();
        match root.value {
            Some(value) => print!("{} ", value),
            None => (),
        }
        root.print_node(0);
    }
}

#[derive(Debug)]
struct Node {
    value: Option<i32>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    fn add_node(mut self, node: Node) -> Tree {
        if node.value < self.value {
            if self.left.is_none() {
                self.left = Some(Box::new(node));
                return Tree {
                    root: Some(Box::new(self)),
                };
            } else {
                let left = self.left.unwrap().add_node(node);
                self.left = Some(Box::new(left.tree_to_node()));
                return Tree {
                    root: Some(Box::new(self)),
                };
            }
        } else if node.value > self.value {
            if self.right.is_none() {
                self.right = Some(Box::new(node));
                return Tree {
                    root: Some(Box::new(self)),
                };
            } else {
                let right = self.right.unwrap().add_node(node);
                self.right = Some(Box::new(right.tree_to_node()));
                return Tree {
                    root: Some(Box::new(self)),
                };
            }
        } else {
            return Tree {
                root: Some(Box::new(self)),
            };
        }
    }

    fn print_node(self, len: i32) {
        match self.left {
            Some(node) => {
                if node.value.unwrap() >= 0 {
                    print!("=> +{} ", node.value.unwrap());
                } else {
                    print!("=> {} ", node.value.unwrap());
                }
                node.print_node(len + 1)
            }
            None => (),
        }
        match self.right {
            Some(node) => {
                println!("");
                for _i in 0..len {
                    print!(" ");
                }
                if node.value.unwrap() >= 0 {
                    print!(" |=> +{} ", node.value.unwrap());
                } else {
                    print!(" |=> {} ", node.value.unwrap());
                }
                node.print_node(len + 1)
            }
            None => (),
        }
    }
}
