use translationhash::TranslationHash;
use translationloader::TranslationLoader;
use std::env;

mod translationhash;
mod translationloader;

fn take_input() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");
    line
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut translationloader: TranslationLoader;
    let mut translationhash: TranslationHash;
    
    if args.len() > 1 {
        translationloader = TranslationLoader {
            path: args[1].clone(),
            count: args[2].parse::<usize>().unwrap(),
        };
        translationhash = translationloader.load();
    } else {
        translationloader = TranslationLoader{path: "".to_string(), count: 0};
        translationhash = TranslationHash::new();
    }

    loop {
        let take_input = take_input();
        let mut input = take_input.split_whitespace();
        let option = input.next().unwrap();

        match option {
            "load" => {
                translationloader = TranslationLoader{path: input.next().unwrap().to_string(), count: input.next().unwrap().to_string().parse::<usize>().unwrap()};
                translationhash = translationloader.load();
            },
            "clear" => translationhash = TranslationHash::new(),
            "add" => {
                let phrase = input.next().unwrap().to_string();
                let translation = input.next().unwrap().to_string();

                translationhash.add(&phrase, &translation);
            },
            "info" => println!("TranslationLoader Info\n  File Path: {}\n  Count: {}\n\nTranslationHash Info\n  Capacity: ({}/{})\n  Load Percent: {}%\n  Load Factor: {}",
                translationloader.path,
                translationloader.count, 
                translationhash.count(), 
                translationhash.size(), 
                translationhash.count() as f32 / translationhash.size() as f32, 
                translationhash.load_factor),
            "find" => {
                let mut inputdata = take_input.chars();
                inputdata.nth(4);
                inputdata.nth_back(1);

                let search = inputdata.as_str().to_string();

                let result = translationhash.at(&search);
                if result.is_none() {
                    println!("Phrase not found")
                } else {
                    println!("{}", result.unwrap())
                }},
            "quit" => break,
            "list" => {
                for i in 0..input.next().unwrap().parse::<usize>().unwrap(){
                    let translation = translationhash.at_index(i);
                    if translation.is_none() {
                        println!("{} (Empty)", i)
                    } else {
                        let data = translation.unwrap();
                        println!("{} {} : {}", i, data[0], data[1])}
                }},
            _=> println!("Invalid Command")
        }
    }
}
