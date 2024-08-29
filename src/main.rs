use crate::tree::Node;

mod tree;

fn main() {

    let contents = std::fs::read_to_string("./res/de-5.txt").unwrap();
    let mut words = contents.split("\n").map(|x| x.split_whitespace().nth(0).unwrap()).collect::<Vec<&str>>();

    let mut t = tree::Tree::new_with(' ');

    words.iter_mut().for_each(|&mut word| {
        let mut cur = &mut t.root;
        for (i, c) in word.chars().enumerate() {
            // if there isn't a word path to follow…
            if !cur.has_child(c) {
                // …create the next letter of the word path
                cur = cur.add_childv(c);
            } else {
                // …follow the word path
                cur = cur.get_child(c).unwrap();
            }
        }
        // and mark the end/stem of a word
        cur.set_stem(true);
    });


    println!("{:#?}", t.root.children);
}
