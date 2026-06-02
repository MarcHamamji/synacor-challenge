use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Node {
    x: u8,
    y: u8,
    value: u16,
    prev: Option<Box<Node>>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.value == other.value
    }
}

impl Node {
    fn new(x: u8, y: u8, value: u16, prev: Option<Box<Node>>) -> Node {
        Node { x, y, value, prev }
    }
}

//              30 (vault)
// * — 8 — - — 1
// |   |   |   |
// 4 — * — 11— *
// |   |   |   |
// + — 4 — - — 18
// |   |   |   |
// 22— - — 9 — *
// ↑

fn main() {
    tracing_subscriber::fmt::init();

    let mut nodes: VecDeque<Node> = VecDeque::from([Node::new(0, 0, 22, None)]);
    let mut worklist: VecDeque<Node> = VecDeque::from([Node::new(0, 0, 22, None)]);

    while !worklist.is_empty() {
        let current_node = worklist.pop_front().unwrap();

        let is_goal = current_node == Node::new(3, 3, 30, None);
        if is_goal {
            print_path(&current_node, &nodes);
            std::process::exit(0);
        }

        let neighbors = get_neighbors(&current_node);
        worklist.extend(neighbors.clone());
        nodes.extend(neighbors);
    }
}

fn print_path(node: &Node, nodes: &VecDeque<Node>) {
    let mut current = node;

    let mut path = VecDeque::new();
    path.push_front(current);

    loop {
        if let Some(prev) = current.prev.as_ref() {
            path.push_front(prev);
            current = nodes.iter().find(|n| (*n).eq(prev)).unwrap();
        } else {
            break;
        }
    }

    println!("===================== PATH TO {:?} ============= LENGTH: {}", node, path.len());
    for node in path {
        println!("{:?}", node);
    }
}

fn get_neighbors(node: &Node) -> Vec<Node> {
    let mut neighbors: Vec<Node> = vec![];

    match (node.x, node.y) {
        (0, 0) => {
            neighbors.push(Node::new(
                0,
                2,
                node.value.overflowing_add(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_add(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_sub(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                0,
                node.value.overflowing_sub(9).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (2, 0) => {
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_sub(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_sub(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                1,
                node.value.overflowing_sub(18).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                1,
                node.value.overflowing_mul(18).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (1, 1) => {
            neighbors.push(Node::new(
                2,
                0,
                node.value.overflowing_sub(9).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                1,
                node.value.overflowing_sub(18).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_sub(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_mul(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                3,
                node.value.overflowing_mul(8).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                0,
                2,
                node.value.overflowing_mul(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                0,
                2,
                node.value.overflowing_add(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (3, 1) => {
            neighbors.push(Node::new(
                2,
                0,
                node.value.overflowing_sub(9).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                0,
                node.value.overflowing_mul(9).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                3,
                node.value.overflowing_mul(1).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_mul(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_sub(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_sub(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (0, 2) => {
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_add(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_mul(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_mul(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                3,
                node.value.overflowing_mul(8).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (2, 2) => {
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_mul(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_sub(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                0,
                node.value.overflowing_sub(9).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                1,
                node.value.overflowing_sub(18).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                1,
                node.value.overflowing_mul(18).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                3,
                node.value.overflowing_mul(1).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                3,
                node.value.overflowing_sub(1).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                3,
                node.value.overflowing_sub(8).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                3,
                node.value.overflowing_mul(8).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                0,
                2,
                node.value.overflowing_mul(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (1, 3) => {
            neighbors.push(Node::new(
                0,
                2,
                node.value.overflowing_mul(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                1,
                1,
                node.value.overflowing_mul(4).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_mul(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                2,
                2,
                node.value.overflowing_sub(11).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
            neighbors.push(Node::new(
                3,
                3,
                node.value.overflowing_sub(1).0,
                Some(Box::new(Node {
                    x: node.x,
                    y: node.y,
                    value: node.value,
                    prev: None,
                })),
            ));
        }
        (3, 3) => {}
        _ => unreachable!(),
    }

    neighbors
}
