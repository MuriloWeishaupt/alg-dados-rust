// ---------------- Usando Reference Counted (Rc) ----------------------------

// use std::rc::Box;

// struct Node {
//     value: i32,
//     next: Option<Box<Node>>,
// }

// impl Node {
//     fn new(value: i32) -> Box<Node> {
//         Box::new(Node {
//             value,
//             next: None,
//         })
//     }

//     fn append(node: &Box<Node>, value: i32) -> Box<Node> {
//         Box::new(Node {
//             value,
//             next: Some(Box::clone(node)),
//         })
//     }
// }


// fn main() {

//     //Cria o primeiro nó
//     let first = Node::new(10);

//     //Cria o segundo nó 
//     let second = Node::append(&first, 20);

//     println!("Primeiro nó: {}", second.next.as_self().unwrap().value);
//     println!("Segundo nó: {}", second.value);
// }




// ----------------------- Estrutura Usando Box -----------------------------

#[derive(Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: None,
        })
    }

    fn append(node: &Box<Node>, value: i32) -> Box<Node> {
        Box::new(Node {
            value,
            next: Some(Box::clone(node)),
        })
    }
}

fn main() {
    let first = Node::new(10);
    let second = Node::append(&first, 20);

    println!("Primeiro nó: {}", second.next.as_ref().unwrap().value);
    println!("Segundo nó: {}", second.value);
}
