use std::time::SystemTime;
use crate::tree::Tree;

mod tree;
mod input;

fn main() {
    let contents = std::fs::read_to_string("./res/en-80k.txt").unwrap();
    let words = contents.split("\n").map(|x| x.split_whitespace().nth(0).unwrap()).collect::<Vec<&str>>();

    // Build tree
    let time = SystemTime::now();
    let t = Tree::<char>::build_from_dict(words);
    println!("Took {}ms to construct the tree", SystemTime::duration_since(&SystemTime::now(), time).unwrap().as_millis());

    // Query suggestions
    //let suggestions = Tree::<char>::complete_sentence(&mut t, "hallo meine kosmon");
    //println!("{:#?}", &suggestions[0..min(5, suggestions.len())]);

    input::track(t);
}
