use std::{collections::HashMap, io};

fn ispalin(s: &str, t: &str)-> bool{
    let mut isPal: bool = true;
    if s.len() != t.len() {
        return false;
    }else{
        let  c1: Vec<char> = s.chars().collect();
        let  c2: Vec<char> = t.chars().collect();
        let mut hash: HashMap<char, i32> = HashMap::new();
        for ch in &c1 {
            *hash.entry(*ch).or_insert(0) += 1;
        }

        for ch in &c2 {
            *hash.entry(*ch).or_insert(0) -= 1;
        }
        for val in hash.values() {
            if  *val != 0 {
                isPal = true;
            }
        }
    }
    isPal
}

fn str_len(mut s: &str)-> usize{
    s = s.trim();
    let  ch:Vec<char> = s.chars().collect();
    let mut ln:usize = 0;
    for _i in 0..ch.len(){
        ln += 1;
    }
    ln
}