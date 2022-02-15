pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    (n * (n + 1) / 2) - nums.iter().sum::<i32>()
}

fn main() {
    println!("{}", missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]))
}
