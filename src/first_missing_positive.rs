pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    for i in 0..nums.len() {
        while nums[i] > 0 && nums[i] <= nums.len() as i32 && nums[(nums[i] - 1) as usize] != nums[i]
        {
            let index = (nums[i] - 1) as usize;

            let aux = nums[i];
            nums[i] = nums[index];
            nums[index] = aux;
        }
    }

    for i in 0..nums.len() {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }

    (nums.len() + 1) as i32
}

fn main() {
    println!("{:?}", first_missing_positive(vec![3, 3, 3, 1]));
}
