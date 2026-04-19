use std::collections::{BinaryHeap, HashMap};


fn freq_sort(str:&str)-> String {
    let mut hash:HashMap<char,i32> = HashMap::new();
    for char in str.chars() {
        *hash.entry(char).or_insert(0) += 1;
    }
    let mut pq: BinaryHeap<(i32,char)> =  BinaryHeap::new();
    for (c, freq) in hash {
        pq.push((freq, c));
    }
    let mut result = String::with_capacity(str.len());
    while let Some((freq, c)) = pq.pop() {
        for _ in 0..freq {
            result.push(c);
        }
    }
    result

}

fn main(){
    let  str: &str = "apple";
    let mut k:i32 = 2;
    let mut map:HashMap<char,i32> = HashMap::new();
    for ch  in str.chars(){
        *map.entry(ch).or_insert(0) += 1;
    }

    let keys = map.keys().len();
    if k == keys as i32 {
        println!("{}",0);
        return;
    }else{
        let mut res:String = freq_sort(str);
        let mut idx:usize  = str.len();
        while k > 0  && idx > 0{
            res.remove(idx);
            idx -= 1;
            k -= 1;
        }
        print!("{}",res);
    }


}

