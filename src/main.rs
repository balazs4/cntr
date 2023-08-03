use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut counter: HashMap<String, i32> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let text = line.unwrap();
        if let Some(count) = counter.get_mut(&text) {
            *count += 1;
            continue;
        }
        counter.insert(text, 1);
    }

    for (line, count) in counter.iter() {
        println!("{}\t{}", count, line);
    }
}
