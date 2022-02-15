use std::{collections::HashSet, vec};

pub fn two_sum(nums: Vec<i32>, sum: i32) -> Vec<Vec<i32>> {
    let mut result: HashSet<Vec<i32>> = HashSet::new();

    for i in 0..nums.len() {
        let searching_slice = &nums[i + 1..];
        if let Ok(index) = searching_slice.binary_search(&(sum - nums[i])) {
            result.insert(vec![nums[i], searching_slice[index]]);
        }
    }

    result.into_iter().collect()
}

pub fn three_sum(nums: Vec<i32>, sum: i32) -> Vec<Vec<i32>> {
    let mut result: HashSet<Vec<i32>> = HashSet::new();

    for i in 0..nums.len() {
        let searching_slice = nums[i + 1..].to_vec();
        let found_two_sum = two_sum(searching_slice, sum - nums[i]);

        for two_sum in found_two_sum {
            result.insert(vec![nums[i], two_sum[0], two_sum[1]]);
        }
    }

    result.into_iter().collect()
}

pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result: HashSet<Vec<i32>> = HashSet::new();
    let mut nums = nums;
    nums.sort();

    for i in 0..nums.len() {
        let searching_slice = nums[i + 1..].to_vec();
        let found_three_sum = three_sum(searching_slice, target - nums[i]);

        for three_sum in found_three_sum {
            result.insert(vec![nums[i], three_sum[0], three_sum[1], three_sum[2]]);
        }
    }

    result.into_iter().collect()
}

fn main() {
    let vec = vec![2, 2, 2, 2, 2];

    println!("{:?}", four_sum(vec, 8));
}
