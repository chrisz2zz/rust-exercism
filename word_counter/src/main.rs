use std::{env, fs::File, io::{prelude::BufRead, BufReader, self, BufWriter}, collections::HashMap};

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> WordCounter {
        WordCounter(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count+=1;
    }

    fn display(&self) {
        let sorted_vec = self.sort();
        for (k, v) in sorted_vec.into_iter().filter(|(_, v)| **v > 1) {
            println!("{}: {}", k, v);
        }
    }

    fn save<W: io::Write>(&self, writer: &mut W)  {
        let sorted_vec = self.sort();
        for (k, v) in sorted_vec.into_iter() {
            let _ = writer.write(format!("{}: {}\n", k, v).as_bytes());
        }
    }

    fn sort(&self) -> Vec<(&String, &u64)> {
        let mut sorted_vec: Vec<(&String, &u64)> = self.0.iter().collect();
        sorted_vec.sort_by(|a, b| a.0.cmp(b.0));
        sorted_vec
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    println!("Processing file: {}", filename);
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();
    
    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");

        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }

    word_counter.display();

    let output = File::create("res.txt").unwrap();
    let mut bw = BufWriter::new(output);
    word_counter.save(bw.get_mut());
}
