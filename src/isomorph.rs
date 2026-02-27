use std::{collections::HashMap, hash::Hash, io};

fn is_iso(s1: &str, s2: &str) -> bool{
    let mut s: HashMap<char,char> = HashMap::new();
    let mut t: HashMap<char, char> = HashMap::new();
    let mut c1: Vec<char> = s1.chars().collect();
    let mut c2: Vec<char> = s2.chars().collect();
    for i in 0..s1.len() {
        if s.contains_key(&c1[i]){
            if s.get(&c1[i]) != Some(&c2[i]) {
                return false;
            }else{
                s.insert(c1[i], c2[i]);
            }
        }
        if t.contains_key(&c2[i]){
            if s.get(&c2[i]) != Some(&c1[i]) {
                return false;
            }else{
                s.insert(c2[i], c1[i]);
            }
        }
    }
    true
}