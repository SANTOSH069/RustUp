mod sum;
mod lapi;
mod intsecarr;
mod isomorph;
mod ispalin;
mod valippairs;
mod intarr;

use std::{collections::HashMap, io,};

fn add_num(a: i32,  b: i32) -> i32 {
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
    let mut nums: Vec<i32>;
    let target: i32;

    // let mut res: Vec<i32> = Solution::two_sum(_nums, target);
    println!("{:?}",res); 
}
    

struct Solution{

}

impl Solution {
    pub fn two_sum(_data: &mut Vec<i32>, target: i32) -> Vec<i32>{
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (idx, val) in _data.iter().enumerate(){
            let complement = target - val;
            if let Some(&complement_idx) = map.get(&complement) {
                return vec![complement_idx as i32, idx as i32];
            }
            map.insert(*val, idx as i32);
        }
        vec![]
    }

    pub fn get_min_distance(_nums: &mut Vec<i32>, _target: i32, start: i32) -> i32 {
        let mut min_distance = i32::MAX;
        for (i, &num) in _nums.iter().enumerate() {
            if num == _target {
                let distance = (i as i32 - start).abs();
                min_distance = min_distance.min(distance);
            }
        }
        if min_distance == i32::MAX { -1 } else { min_distance }
    }

    pub fn maj_ele(_nums: &mut Vec<i32>) -> i32 {
        let mut hash: HashMap<i32, i32>  = HashMap::new();
        let n: i32  = _nums.len() as i32;
        for &num in _nums.iter() {
            let count = hash.entry(num).or_insert(0);
            *count += 1;

            if *count > (n / 2) {
                return num;
            }
        }
        -1
    }

    pub fn max_ones(ones: Vec<i32>) -> i32 {
        let mut curr_ones: i32 = 0;
        let mut max_ones: i32 = i32::MAX; 
        for num in 0..ones.len() {
            if num as i32 == 1 {
                curr_ones += 1;
            }else {
                max_ones = std::cmp::max(max_ones, curr_ones);
                curr_ones = 0;
            }
        }
        return max_ones;
    }

    pub fn med_arr(nums: &Vec<i32>, left: usize, right: usize) -> i32 {
        let mut sl = nums[left..=right].to_vec();
        sl.sort();

        let len = sl.len();
        if len == 0 {
            return 0;
        }

        if len % 2 == 1 {
            sl[len / 2]
        } else {
            (sl[len / 2 - 1] + sl[len / 2]) / 2
        }
    }

    pub fn sub_med(nums: &mut Vec<i32>, k: i32) -> i32 {
        let mut count = 0;

        for left in 0..nums.len() {
            for right in left..nums.len() {
                if Self::med_arr(nums, left, right) == k {
                    count += 1;
                }
            }
        }

        count
    }
}



