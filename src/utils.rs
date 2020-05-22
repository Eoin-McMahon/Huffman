use std::fs;

// strip new line characters
pub fn remove_newlines(mut raw_string: String) -> String {
    let len = raw_string.trim_end_matches(&['\r', '\n'][..]).len();
    raw_string.truncate(len);

    return raw_string;
}

// read file passed as argument and return as String
pub fn retrieve_string_from_file(args: Vec<String>) -> String {
    let filename: &String = &args[1];
    let raw = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    return raw;
}

// calculate size difference between original and encoded and print to screen
pub fn output_stats(encoded_string: String, initial_string: String) {

    // calculate size decrease
    let initial_size: f64 = (initial_string.len() * 8) as f64;
    let encoded_size: f64 = encoded_string.len() as f64;
    let size_decrease: f64 = ((initial_size - encoded_size) / initial_size) * 100.0;


    println!("\n\ninitial size: {} bits\nencoded_size: {} bits\nsize decrease: {:.2}%",
             initial_size, encoded_size, size_decrease)

}