use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct Node {
    ch: Option<char>,
    freq: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // allocate memory on heap for node
    fn to_box(self) -> Box<Node> {
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

fn build_frequency_map(s: &str) -> HashMap<char, i32> {
    let mut freq_map = HashMap::new();
    for ch in s.chars() {
        // get count from hashmap otherwise add 0 as the v
        let count = freq_map.entry(ch).or_insert(0);
        *count += 1;
    }

    return freq_map;
}


fn build_huffman_tree(mut nodes: Vec<Box<Node>>) -> Box<Node> {
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
                                    .map(|x| Node::new(*(x.1), Some(*(x.0))).to_box())
                                    .collect();

    return nodes;
}


fn encode_string(str: &str, code_table: &HashMap<char, String>) -> String {
    let mut encoded = String::from("");
    let mut code:Option<&String>;

    for ch in str.chars() {
        // push character code to output string
        code = code_table.get(&ch);
        encoded.push_str(code.unwrap());
    }
    
    return encoded;
}

fn decode_string(str: &str, root: &Box<Node>) -> String {
    let mut decoded = String::from("");
    let mut node = root;

    for num in str.chars() {
        if num == '0' {
            if let Some(ref l) = node.left {
                node = l;
            }
        }
        else {
            if let Some(ref r) = node.right {
                node = r;
            }
        }

        if let Some(ch) = node.ch {
            decoded.push(ch);
            node = root;
        }
    }

    return decoded;
}

fn main() {
    println!("String to be encoded:");

    let mut raw = String::new();

    io::stdin()
        .read_line(&mut raw)
        .expect("Failed to read string");
    
    // trim newline from string
    let len = raw.trim_end_matches(&['\r', '\n'][..]).len();
    raw.truncate(len);

    let freqs = build_frequency_map(&raw);
    let empty_string:String = String::from("");
    
    let nodes:Vec<Box<Node>> = build_node_vector(freqs);
    let root:Box<Node> = build_huffman_tree(nodes);

    let mut code_table:HashMap<char, String> = HashMap::new();
    assign_codes(&root, &mut code_table, empty_string);

    let encoded:String = encode_string(&raw, &code_table);
    println!("\nEncoded string: {:?}", encoded);


    let decoded:String = decode_string(&encoded, &root);
    println!("Decoded string: {:?}", decoded);

    // calculate compression ratio
    let initial_size: f64 = (raw.len() * 8) as f64;
    let encoded_size: f64 = encoded.len() as f64;
    let compression_ratio: f64 = ((initial_size - encoded_size) / initial_size) * 100.0;

    println!("\n\ninitial size: {}\nencoded_size: {}\ncompression ratio: {:.2}%",
             initial_size, encoded_size, compression_ratio)
}








