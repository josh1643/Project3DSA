use crate::translationhash::*;
use std::fs::read_to_string;

pub struct TranslationLoader {
    pub path: String,
    pub count: usize,
}

impl TranslationLoader {
    pub fn load(&self) -> TranslationHash {
        let mut translationhash = TranslationHash::new();
        let mut lines: Vec<String> = read_to_string(self.path.clone())
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect();

        if self.count != 0 {
            lines = lines
                .iter()
                .take(self.count)
                .map(|line| line.to_string())
                .collect();
        }

        for line in lines {
            if line.chars().filter(|char| *char == ',').count() == 1 {
                let mut linedata = line.split(",");

                let phrase = linedata.next().unwrap().to_string();
                let translation = linedata.next().unwrap().to_string();

                translationhash.add(&phrase, &translation);
            }
        }
        translationhash
    }
}
