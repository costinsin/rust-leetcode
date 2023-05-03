pub fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        return 0;
    }

    let negative = dividend.is_negative() ^ divisor.is_negative();

    let mut dividend = (dividend as i64).abs();
    let divisor = (divisor as i64).abs();

    let mut result: i64 = 0;

    while dividend >= divisor {
        dividend -= divisor;

        result += 1;
    }

    (result * if negative { -1 } else { 1 }).min(2147483647).max(-2147483648) as i32
}
