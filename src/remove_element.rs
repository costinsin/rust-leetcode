pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut last_available_index = nums.len() as i32 - 1;

    let mut i = 0;
    while i < last_available_index {
        if nums[i as usize] == val {
            nums.swap(i as usize, last_available_index as usize);
            last_available_index -= 1;
        } else {
            i += 1;
        }
    }

    if last_available_index >= 0 && nums[last_available_index as usize] == val {
        last_available_index as i32
    } else {
        (last_available_index + 1) as i32
    }
}

fn main() {
    let mut nums = vec![1];
    nums.len();
    println!("{} - {:?}", remove_element(&mut nums, 1), nums);
}
