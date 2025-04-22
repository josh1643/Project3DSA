#![allow(unused)]
use std::vec;

pub struct TranslationHash {
    translations: TranslationMap,
    pub load_factor: f32,
}

struct TranslationNode {
    phrase: String,
    translation: String,
}

struct TranslationMap {
    translations: Vec<Option<TranslationNode>>,
    size: i32,
    count: i32,
}

impl TranslationMap {
    fn new(size: i32) -> Self {
        let mut translations = Vec::new();
        for _ in 0..size {
            translations.push(None);
        }
        TranslationMap {
            translations,
            size,
            count: 0,
        }
    }

    fn hash(&self, key: &String) -> usize {
        key.clone()
            .chars()
            .map(|letter| letter.to_ascii_lowercase() as usize)
            .sum::<usize>()
            % self.translations.len()
    }

    fn add(&mut self, phrase: &String, translation: &String) {
        let hash_key = self.hash(phrase);
        let mut offset = 0;
        loop {
            let index = (hash_key + (offset * offset * offset)) % self.size as usize;
            if self.translations[index].is_none() {
                self.translations[index] = Some(TranslationNode {
                    phrase: phrase.clone(),
                    translation: translation.clone(),
                });
                self.count += 1;
                break;
            } else {
                let nodedata = self.translations[index].as_ref().unwrap();

                if phrase.clone() == nodedata.phrase {
                    self.translations[index] = Some(TranslationNode {
                        phrase: phrase.clone(),
                        translation: translation.clone(),
                    });
                    self.count += 1;
                    break;
                } else {
                    offset += 1;
                }
            }
        }
    }

    fn at(&self, phrase: &String) -> Option<String> {
        let hash_key = self.hash(phrase);
        let mut offset = 0;
        loop {
            let index = (hash_key + (offset * offset * offset)) % self.size as usize;
            if self.translations[index].is_none() {
                return None;
            } else {
                let nodedata = self.translations[index].as_ref().unwrap();

                if phrase.clone() == nodedata.phrase {
                    return Some(nodedata.translation.clone());
                } else {
                    offset += 1;
                }
            }
        }
    }
}

impl TranslationHash {
    pub fn new() -> Self {
        TranslationHash {
            translations: TranslationMap::new(8),
            load_factor: 0.7,
        }
    }

    pub fn add(&mut self, phrase: &String, translation: &String) {
        let lowercase = phrase
            .chars()
            .map(|char| char.to_ascii_lowercase())
            .collect::<String>();

        self.translations.add(&lowercase, translation);
        if self.translations.count as f32 / self.translations.size as f32 >= self.load_factor {
            self.rehash();
        }
    }

    fn rehash(&mut self) {
        let mut translations_new: TranslationMap = TranslationMap::new(self.translations.size * 2);
        for node in &self.translations.translations {
            if node.is_some() {
                let nodedata = node.as_ref().unwrap();
                translations_new.add(&nodedata.phrase, &nodedata.translation);
            }
        }
        self.translations = translations_new;
    }

    pub fn at(&self, key: &String) -> Option<String> {
        let lowercase = key
            .chars()
            .map(|char| char.to_ascii_lowercase())
            .collect::<String>();

        self.translations.at(&lowercase)
    }

    pub fn size(&self) -> i32 {
        self.translations.size
    }

    pub fn count(&self) -> i32 {
        self.translations.count
    }

    pub fn at_index(&self, index: usize) -> Option<Vec<String>> {
        if self.translations.translations[index].is_none() {
            None
        } else {
            let node = self.translations.translations[index].as_ref().unwrap();
            Some(vec![node.phrase.clone(), node.translation.clone()])
        }
    }
}
