use std::collections::HashMap;
use std::fs::read_to_string;
//use std::vec::Vec;

pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    translation: String,
}

impl TrieNode {
    fn default() -> Self {
        Self {
            children: HashMap::new(),
            translation: String::new(),
        }
    }
    fn go_to(&mut self, c: char) -> &mut TrieNode {
        if !self.children.contains_key(&c) {
            self.children.insert(c, TrieNode::default());
        }
        self.children.get_mut(&c).unwrap()
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn default() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }
    pub fn load_from_file(&mut self, file_path: String) {
        //let mut counter = 0;
        for line in read_to_string(file_path).unwrap().lines() {
            if line.chars().filter(|c| *c == ',').count() != 1 {
                continue;
            }
            let english = line.split(',').next().unwrap();
            let english_lower = english.to_lowercase();
            let spanish = line.split(',').last().unwrap();
            let mut current = &mut self.root;
            for c in english_lower.chars() {
                current = current.go_to(c);
            }
            current.translation = spanish.to_string();
            //counter += 1;
        }
        //println!("Loaded {} words", counter);
    }
    pub fn translate(&self, word: &str) -> String {
        let mut current = &self.root;
        let word_lower = word.to_lowercase();
        for c in word_lower.chars() {
            current = &current.children[&c];
        }
        return current.translation.clone();
    }
}
/*
fn main() {
    let file_path = "C:/Users/Daniel/Desktop/project3COP/words.txt";
}
*/
