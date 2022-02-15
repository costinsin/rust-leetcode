use std::collections::HashSet;

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut set = HashSet::new();

    for num in nums.iter() {
        if let Some(_) = set.get(num) {
            set.remove(num);
        } else {
            set.insert(num);
        }
    }

    *set.into_iter().next().unwrap()
}

fn main() {
    println!("{}", single_number(vec![4, 1, 2, 1, 2]))
}
