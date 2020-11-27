use std::collections::HashMap;

fn main() {
    // Vectors have their own methods and are basically like arrays.
    let mut my_vec: Vec<i32> = Vec::new();

    let my_vec_from_macro = vec![1, 2, 3, 4];
    //let dne = my_vec_from_macro[100];

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue Team"), 89);
    scores.insert(String::from("Red Team"), 10);

    // The Key and Value can be iterated over a for loop like so: 
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    // VERY POWERFUL EXAMPLE OF HASH MAP AND .entry() function to count 
    // the number of occurences of a word in a string
    
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
