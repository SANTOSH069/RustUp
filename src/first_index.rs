use std::io;



fn  main() {
    let mut input:String = String::new();
    io::stdin().read_line(&mut input).expect("Error Taking input");
    let n:i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default 0.");
            0
        }
    };
    let mut nums:Vec<i32> = Vec::new();
    for i in 0..n {
        let  x:i32 = match input.trim().parse() {
           Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Using default 0.");
                0
            } 
        };
        nums.push(x);
    }
    let  k:i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default 0.");
            0
        }
    };
    let res:i32 = first_index(nums, k);
    println!("{}",res);
}

pub fn first_index(nums: Vec<i32>, k: i32) -> i32 {
    let mut res: Vec<i32> = Vec::new();  // initialize vector

    for i in 0..nums.len() {
        let diff: i32 = index_diff(&nums, i);  // pass reference, not ownership

        if diff <= k {
            res.push(diff);  // use push instead of append
        }
    }

    if res.is_empty() {
        return -1;
    }

    let ans: i32 = *res.iter().min().unwrap();  
    ans
}

pub fn index_diff(nums: &Vec<i32>, i: usize) -> i32 {
    if i == 0 || i == nums.len() {
        return i32::MAX; 
    }

    let beg = &nums[0..i];
    let end = &nums[i..nums.len()];

    let max_beg = beg.iter().max().unwrap();
    let min_end = end.iter().min().unwrap();

    *max_beg - *min_end
}