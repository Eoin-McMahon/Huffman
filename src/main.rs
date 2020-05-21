use std::collections::HashMap;
use std::fmt;

struct Node {
    ch: Option<char>,
    freq: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn to_box(self) -> Box<Node> {
        return Box::new(self);
    }
}

fn frequency_map(s: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();
    for ch in s.chars() {
        let count = freq_map.entry(ch).or_insert(0); // get count from hashmap otherwise add 0
        *count += 1;
    }

    return freq_map;
}

fn new_node(freq: i32, ch: Option<char>) -> Node {
    return Node {
        freq,
        ch,
        left: None,
        right: None,
    };
}

fn build_huffman_tree(mut nodes: Vec<Box<Node>>) -> Box<Node> {
    for node in nodes.iter() {
    println!("{:?} {}",node.ch, node.freq);
    }
    while nodes.len() > 1 {
        nodes.sort_by(|x, y| (&(x.freq)).cmp(&(y.freq)));
        let x = nodes.pop().unwrap();
        println!("char: {:?}\t freq: {}",x.ch, x.freq);
        let y = nodes.pop().unwrap();
        println!("char: {:?}\t freq: {}",y.ch,  y.freq);
        let mut z = new_node(x.freq + y.freq, None).to_box();

        z.left = Some(x);
        z.right = Some(y);
        nodes.push(z);
    }

    // return root of huffman tree
    let root = nodes.pop().unwrap();
    return root;
}

fn assign_codes(node: &Box<Node>, code_table: &mut HashMap<char, String>, code: String) {
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

fn build_node_vector(freqs: HashMap<char, i32>) -> Vec<Box<Node>> {
    // map over k, v tuples and create nodes for them.
    let nodes:Vec<Box<Node>> = freqs.iter()
                                    .map(|x| new_node(*(x.1), Some(*(x.0))).to_box())
                                    .collect();

    return nodes;
}


fn encode_string(str: &str, code_table: &HashMap<char, String>) -> String {
    let mut encoded = String::from("");
    let mut code:Option<&String>;

    for ch in str.chars() {
        code = code_table.get(&ch);
        encoded.push_str(code.unwrap());
    }
    
    return encoded;
}


fn main() {
    let raw: String = String::from("my name is eoin mcmahon");

    let freqs = frequency_map(&raw);
    let empty_string:String = String::from("");
    println!("{:?}", freqs);
    
    let nodes:Vec<Box<Node>> = build_node_vector(freqs);
    let root:Box<Node> = build_huffman_tree(nodes);

    let mut code_table:HashMap<char, String> = HashMap::new();
    assign_codes(&root, &mut code_table, empty_string);
    println!("{:?}", code_table);
    let encoded:String = encode_string(&raw, &code_table);
    println!("{:?}", encoded);


}








