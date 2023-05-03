pub fn my_pow(x: f64, n: i32) -> f64 {
    if n == 0 || x == 1.0 {
        return 1.0;
    }

    let mut pow = 1 as f64;
    let mut aux = x;
    let op = if n < 0 {
        |x: f64, y: f64| x / y
    } else {
        |x: f64, y: f64| (x * y) as f64
    };
    let n = if n < 0 { -(n as i64) } else { n as i64 };
    let mut n_power = n;

    while n_power != 0 {
        if n_power & 1 == 1 {
            pow = op(pow, aux);
        }
        n_power = n_power >> 1;
        aux *= aux;
    }

    pow
}
