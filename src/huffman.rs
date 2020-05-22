use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub ch: Option<char>,
    pub freq: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    // allocate memory on heap for node
    pub fn to_box(self) -> Box<Node> {
        return Box::new(self);
    }

    // constructor for a new node
    pub fn new(freq: i32, ch: Option<char>) -> Self {
        return Self {
            ch: ch,
            freq: freq,
            left: None,
            right: None,
        };
    }
}

pub fn build_frequency_map(s: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();
    for ch in s.chars() {
        // get count from hashmap otherwise add 0 as the v
        let count = freq_map.entry(ch).or_insert(0);
        *count += 1;
    }

    return freq_map;
}


pub fn build_huffman_tree(mut nodes: Vec<Box<Node>>) -> Box<Node> {
    // reduce list down to the root node
    while nodes.len() > 1 {
        nodes.sort_by(|x, y| (&(y.freq)).cmp(&(x.freq)));
        // form subtree for x and y
        let x = nodes.pop().unwrap();
        let y = nodes.pop().unwrap();
        // parent has childrens combined frequency and no associated character
        let mut z = Node::new(x.freq + y.freq, None).to_box();
        z.left = Some(x);
        z.right = Some(y);
        
        nodes.push(z);
    }

    // return root of huffman tree for traversal
    let root = nodes.pop().unwrap();
    return root;
}

pub fn assign_codes(node: &Box<Node>, code_table: &mut HashMap<char, String>, code: String) {
    if let Some(ch) = node.ch {
        code_table.insert(ch, code);
    }
    else {
        if let Some(ref l) = node.left {
            assign_codes(l, code_table, code.clone() + "0");
        }

        if let Some(ref r) = node.right {
            assign_codes(r, code_table, code.clone() + "1");
        }
    }
}

pub fn build_node_vector(freqs: HashMap<char, i32>) -> Vec<Box<Node>> {
    // map over k, v tuples and create nodes for them.
    let nodes:Vec<Box<Node>> = freqs.iter()
                                    .map(|x| Node::new(*(x.1), Some(*(x.0))).to_box())
                                    .collect();

    return nodes;
}