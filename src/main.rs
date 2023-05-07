use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut counter: HashMap<String, i32> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if let Some(value) = counter.get_mut(&l) {
            *value += 1;
            continue;
        }
        counter.insert(l, 1);
    }

    for (key, value) in counter.iter() {
        println!("{}\t{}", key, value);
    }
}
