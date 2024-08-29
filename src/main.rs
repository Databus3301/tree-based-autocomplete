use crate::tree::{Node, Tree};

mod tree;

fn main() {
    let contents = std::fs::read_to_string("./res/de-100k.txt").unwrap();
    let words = contents.split("\n").map(|x| x.split_whitespace().nth(0).unwrap()).collect::<Vec<&str>>();

    // Build tree
    let time = std::time::SystemTime::now();
    let mut t = Tree::<char>::build_from_dict(words);
    println!("Took {}ms to construct the tree", std::time::SystemTime::duration_since(&std::time::SystemTime::now(), time).unwrap().as_millis());

    let suggestions = Tree::<char>::complete(&mut t, "uns");

    println!("{:#?}", suggestions);
}

