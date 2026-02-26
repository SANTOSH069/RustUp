mod sum;
mod lapi;

use std::{io, thread::current};

fn add_num(a: i32, b: i32) -> i32 {
    a + b
}

fn is_sub(s: &str, t: &str) -> bool {
    let mut s_chars = s.chars();
    let mut current = s_chars.next();

    for ch in t.chars() {
        if Some(ch) == current {
            current = s_chars.next();
        }
    }

    current.is_none()
}

fn sum_arr(arr: &mut [i32]) -> i32 {
    let mut sum: i32 = 0;

    for num in arr {
        sum += *num;
    }
    sum
}


fn maj_ele(arr: &mut [i32]) -> i32 {
    let mut freq: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for num in arr {
        *freq.entry(*num).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut majority = 0;
    for (&num, &count) in freq.iter() {
        if count > max_count {
            max_count = count;
            majority = num;
        }
    }
    majority
}
fn lin_search(nums: &[i32],  target: i32) -> bool {
    for num in nums {
        if *num == target {
            return true;
        }
    }
    return false;
}
fn bin_search(nums: &[i32], target: i32) -> bool {
    let mut  left = 0;
    let mut right = nums.len() - 1;
    while left <= right {
        let  mid = left + (right - left)/2;
        if nums[mid] == target {
            return  true;
        }else if nums[mid] < target {
            left = mid + 1;
        }else {
            right = mid - 1;
        }
    }
    false
}
fn main() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error taking input.");

    let a: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default 0.");
            0
        }
    };

    input.clear(); 

    println!("Enter second number:");
    io::stdin()
        .read_line(&mut input)
        .expect("Error taking input.");

    let b: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default 0.");
            0
        }
    };

    let res = add_num(a, b);
    println!("Result: {}", res);

    let arr = [1,2,3,4,5];
    let first = arr[0];
    print!("{}",first);

    let _tp = (1,"Jhon Doe", 169.6);
    
    let mut  stack: Vec<i32> = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    for i in stack {
        println!("{}", i);
    }
}
