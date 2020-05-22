use std::env;
use std::collections::HashMap;
mod huffman;
mod utils;

// encode the string
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

// decode the bytestring
fn decode_string(str: &str, root: &Box<huffman::Node>) -> String {
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
    // read file and clean it
    let args: Vec<String> = env::args().collect();
    let file_string = utils::retrieve_string_from_file(args);
    let contents = utils::remove_newlines(file_string);

    // build huffman tree from text file
    let freqs = huffman::build_frequency_map(&contents);
    let nodes:Vec<Box<huffman::Node>> = huffman::build_node_vector(freqs);
    let root:Box<huffman::Node> = huffman::build_huffman_tree(nodes);

    // assign codes for characters in string
    let mut code_table:HashMap<char, String> = HashMap::new();
    huffman::assign_codes(&root, &mut code_table, String::from(""));

    let encoded:String = encode_string(&contents, &code_table);
    let _decoded:String = decode_string(&encoded, &root);

    // print stats for user
    utils::output_stats(encoded, contents);

    // TODO - write encoded string and tree to file

}








