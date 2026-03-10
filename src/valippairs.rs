use std::{collections::HashMap, io};

fn main(){
    let mut inp: String = String::new();
    let mut n:i32;

    io::stdin()
        .read_line(&mut inp)
        .expect("Error while reading input.");

    // n = inp.trim().parse().expect("Invalid number");

    inp.clear();

    io::stdin()
        .read_line(&mut inp)
        .expect("Error while reading numbers");

    let nums: Vec<i32> = inp
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    inp.clear();

    io::stdin()
        .read_line(&mut inp)
        .expect("Error while reading target");

    let target: i32 = inp.trim().parse().expect("Error while taking input");

    let mut res: Vec<i32> = Vec::new();

    let mut hash: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        *hash.entry(nums[i]).or_insert(0) += 1;
    }

    for (key, value) in hash.iter() {
        if *key == target {
            res.push(*value);
        }
    }

    for num in 0..res.len(){
        print!("{:?} ", res[num]);
    }
}