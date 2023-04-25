use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut counter: HashMap<String, i32> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        match counter.get_mut(&l) {
            Some(value) => *value += 1,
            None => {
                counter.insert(l, 1);
                ()
            }
        };
    }

    for (key, value) in counter.iter() {
        println!("{}\t{}", key, value);
    }
}
