pub fn divide(dividend: i32, divisor: i32) -> i32 {
    let dividend = dividend as i64;
    let divisor = divisor as i64;

    match dividend / divisor {
        2147483648.. => 2147483647,
        i64::MIN..=-2147483649 => -2147483648,
        x => x as i32,
    }
}

fn main() { 
    println!("{}", divide(-2147483648, -1));
}
