use std::collections::HashMap;


fn frequency_map(s: &str) -> HashMap<char, i32> {
    let mut h = HashMap::new();
    for ch in s.chars() {
        let count = h.entry(ch).or_insert(0); // get count from hashmap otherwise add 0
        *count += 1;
    }

    return h;
}


fn main() {
    let freqs = frequency_map("eoin mcmahon");
    println!("{:?}", freqs);
}
