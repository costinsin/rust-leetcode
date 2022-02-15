pub fn int_to_roman(num: i32) -> String {
    let mut result = "".to_string();
    let mut num = num;

    while num > 0 {
        match num {
            x if x >= 1000 => {
                result.push_str("M");
                num -= 1000;
            }
            x if x >= 900 => {
                result.push_str("C");
                num += 100;
            }
            x if x >= 500 => {
                result.push_str("D");
                num -= 500;
            }
            x if x >= 400 => {
                result.push_str("C");
                num += 100;
            }
            x if x >= 100 => {
                result.push_str("C");
                num -= 100;
            }
            x if x >= 90 => {
                result.push_str("X");
                num += 10;
            }
            x if x >= 50 => {
                result.push_str("L");
                num -= 50;
            }
            x if x >= 40 => {
                result.push_str("X");
                num += 10;
            }
            x if x >= 10 => {
                result.push_str("X");
                num -= 10;
            }
            x if x >= 9 => {
                result.push_str("I");
                num += 1;
            }
            x if x >= 5 => {
                result.push_str("V");
                num -= 5;
            }
            x if x >= 4 => {
                result.push_str("I");
                num += 1;
            }
            x if x >= 1 => {
                result.push_str("I");
                num -= 1;
            }
            _ => num = 0,
        }
    }

    result
}

fn main() {
    println!("{}", int_to_roman(1994));
}
