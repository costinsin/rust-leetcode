pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut first, mut second) = (0, 1);

    while second < nums.len() {
        if nums[first] != nums[second] {
            first += 1;
            nums[first] = nums[second];
        }

        second += 1;
    }

    (first + 1) as i32
}

fn main() {
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

    println!("{} - {:?}", remove_duplicates(&mut v), v);
}
