
use std::{collections::HashSet, io};

fn main(){
    let mut set: HashSet<i32> = HashSet::new();
    let mut nums: Vec<i32> = Vec::new();
    let mut inp = String::new();
    io::stdin()
    .read_line(&mut inp)
    .expect("Fialed to Read Input Data");
    let mut x:i32 = match inp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default 0.");
            0
        }
    };
    while x > 0 {
        let val:i32 = match inp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default 0.");
            0
        }
    };
        nums.push(val);
        x -= 1;
    }
    for num in nums {
        set.insert(num);
    }
    println!("{:?}",set);
}