use std::cmp;

//Definição de um nó na árvore
#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: i32, //Altura do nó
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
            height: 1,
        }
    }

    fn height(node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(n) => n.height,
            None => 0,
        }
    }

    fn balance_factor(&self) -> i32 {
        Node::height(&self.left) - Node::height(&self.right)
    }

    fn update_height(&mut self) {
        self.height = cmp::max(Node::height(&self.left), Node::height(&self.right)) + 1;
    }

    fn rotate_right(mut self) -> Box<Node> {
        let mut left_child = self.left.take().unwrap();
        self.left = left_child.right.take();
        self.update_height();
        left_child.right = Some(Box::new(self));
        left_child.update_height();
        left_child
    }

    fn rotate_left(mut self) -> Box<Node> {
        let mut right_child = self.right.take().unwrap();
        self.right = right_child.left.take();
        self.update_height();
        right_child.left = Some(Box::new(self));
        right_child.update_height();
        right_child
    }

    fn balance(mut self) -> Box<Node> {
        self.update_height();
        let balance_factor = self.balance_factor();

        if balance_factor > 1 {
            if let Some(left) = self.left.take() {
                if left.balance_factor() < 0 {
                    self.left = Some(left.rotate_left());
                } else {
                    self.left = Some(left);
                }
            }
            return self.rotate_right();
        }

        if balance_factor < -1 {
            if let Some(right) = self.right.take() {
                if right.balance_factor() > 0 {
                    self.right = Some(right.rotate_right());
                } else {
                    self.right = Some(right);
                }
            }
            return self.rotate_left();
        }

        Box::new(self)
    }

    fn insert(mut self, value: i32) -> Box<Node> {
        if value < self.value {
            self.left = match self.left.take() {
                Some(left) => Some(left.insert(value)),
                None => Some(Box::new(Node::new(value))),
            };
        } else {
            self.right = match self.right.take() {
                Some(right) => Some(right.insert(value)),
                None => Some(Box::new(Node::new(value))),
            };
        }

        self.balance()
    }

    fn search(&self, value: i32) -> bool {
        if value == self.value {
            true
        } else if value < self.value {
            match &self.left {
                Some(left) => left.search(value),
                None => false,
            }
        } else {
            match &self.right {
                Some(right) => right.search(value),
                None => false,
            }
        }
    }
}

#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = match self.root.take() {
            Some(root) => Some(root.insert(value)),
            None => Some(Box::new(Node::new(value))),
        };
    }

    fn search(&self, value: i32) -> bool {
        match &self.root {
            Some(root) => root.search(value),
            None => false,
        }
    }
}

fn main() {
    let mut tree = Tree::new();

    tree.insert(10);
    tree.insert(20);
    tree.insert(30);
    tree.insert(40);
    tree.insert(50);
    tree.insert(25);

    println!("Busca por 20: {}", tree.search(20)); // true
    println!("Busca por 15: {}", tree.search(15)); // false

    println!("{:#?}", tree);
}
