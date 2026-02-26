use std::{collections::HashMap, io};

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take string input.");

    let input = input.trim(); 
    let n = input.len();
    let mid = n / 2;

    let ch: Vec<char> = input.chars().collect();

    let mut hash: HashMap<char, i32> = HashMap::new();

    for i in 0..mid {
        *hash.entry(ch[i]).or_insert(0) += 1;
    }

    for i in mid+1..input.len() {
        *hash.entry(ch[i]).or_insert(0) -= 1;
    }
    let res: bool = hash.values().all(|&val| val == 0);
    println!("{}",res);

}