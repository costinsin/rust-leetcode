use std::cmp;

pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());

    let (mut result, mut left, mut right) = (0, -1, -1);
    for interval in intervals {
        if interval[0] > left && interval[1] > right {
            left = interval[0];
            result += 1;
        }

        right = cmp::max(right, interval[1]);
    }

    result
}

fn main() {
    let v = vec![vec![3, 10], vec![4, 10], vec![5, 11]];

    println!("{}", remove_covered_intervals(v));
}
