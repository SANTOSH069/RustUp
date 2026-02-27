use std::io;


fn intArr(mut s: Vec<i32>, mut t: Vec<i32>) -> Vec<i32> {
    let mut  res  = Vec::new();
    s.sort();
    t.sort();
    for num in s {
        if t.contains(&num) {
            res.push(num);
        }
    }
    res
}
fn main(){
    let mut  inp = String::new();
    io::stdin()
    .read_line(&mut inp)
    .expect("Failed to read input");

    let mut s: Vec<i32> = Vec::new();
    let mut t: Vec<i32> = Vec::new();
    


}