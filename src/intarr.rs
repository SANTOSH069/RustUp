

fn main() {
    let  a: i32 = 10;
    let  b = &a;
    let  c: i32 = *b;
    println!("{}",c);
    println!("{}",b);
}