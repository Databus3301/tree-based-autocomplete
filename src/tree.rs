use std::fmt::format;

#[derive(Debug)]
pub struct Tree<T: PartialEq> {
    pub root: Node<T>
}

#[derive(Debug)]
pub struct Node<T:  PartialEq> {
    value: T,
    stem: bool,
    children: Vec<Node<T>>
}

impl <T:  PartialEq> Tree<T> {
    pub fn new_with(val: T) -> Tree<T> {
        Tree {
            root: Node::new(val)
        }
    }
    pub fn build_from_dict(mut words: Vec<&str>) -> Tree<char> {
        let mut t = Tree::new_with(' ');

        // Build the tree
        words.iter_mut().for_each(|&mut word| {
            let mut cur = &mut t.root;
            for c in word.chars() {
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

        t
    }

    pub fn complete_word(tree: &mut Tree<char>, prefix: &str) -> Vec<String> {
        let mut cur = &mut tree.root;
        let mut path  = String::new();
        for c in prefix.to_ascii_lowercase().chars() {
            // if there is a word path to follow…
            if cur.has_child(c) {
                // …follow the word path
                cur = cur.get_child(c).unwrap();
                path.push(c);
            } else {
                // …don't suggest anything
                return vec!();
            }
        }

//        // collect word ends
//        let mut words = vec!();
//        for child in cur.children.iter_mut() {
//            let mut suffix = String::new();
//
//            let mut tail = child;
//            while !tail.children.is_empty() {
//                suffix.push(tail.value);
//                if tail.stem {
//                    words.push(format!("{}{}", prefix, suffix));
//                }
//                tail = tail.children.first_mut().unwrap();
//            }
//            suffix.push(tail.value);
//
//            words.push(format!("{}{}", prefix, suffix));
//        }
//
//        words
        Self::complete_word_r(cur, "", &path)
    }
    pub fn complete_word_r(mut node: &mut Node<char>, suffix: &str, prefix: &str) -> Vec<String> {
        if node.children.is_empty() {
            return vec!(format!("{}{}", prefix, suffix));
        }

        let mut r = vec!();
        if node.stem {
            r.push(format!("{}{}", prefix, suffix));
        }
        for child in node.children.iter_mut() {
            r.append(&mut Self::complete_word_r(child, &format!("{}{}", suffix, child.value), prefix));
        }


        return r;
    }
    pub fn complete_sentence(tree: &mut Tree<char>, sentence: &str) -> Vec<String> {
        if sentence.len() > 0 {
            Tree::<char>::complete_word(tree, sentence.trim().split_whitespace().last().unwrap())
        } else {
            vec!()
        }
    }
}


impl <T: PartialEq> Node<T> {
    pub fn new(val: T) -> Node<T> {
        Node {
            value: val,
            stem: false,
            children: Vec::new()
        }
    }
    pub fn add_childv(&mut self, val: T) -> &mut Node<T> {
        let n = Node::new(val);
        self.children.push(n);
        self.children.last_mut().unwrap()
    }
    pub fn set_stem(&mut self, b: bool) {
        self.stem = b;
    }
    pub fn has_child(&self, val:T) -> bool {
        self.children.iter().find(|&n| n.value == val).is_some()
    }
    pub fn get_child(&mut self, val: T) -> Option<&mut Node<T>> {
        self.children.iter_mut().find(|n| n.value == val)
    }

}



