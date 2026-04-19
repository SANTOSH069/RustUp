

fn  main() {
    
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
        return i32::MAX; // avoid empty slice panic
    }

    let beg = &nums[0..i];
    let end = &nums[i..nums.len()];

    let max_beg = beg.iter().max().unwrap();
    let min_end = end.iter().min().unwrap();

    *max_beg - *min_end
}