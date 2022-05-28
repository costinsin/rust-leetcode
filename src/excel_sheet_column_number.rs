pub fn title_to_number(column_title: String) -> i32 {
    let mut result = 0;
    let mut string_len = column_title.len() as i32 - 1;
    for c in column_title.as_bytes() {
        result += (1 + *c - 'A' as u8) as i32 * (26 as i32).pow(string_len as u32);
        string_len -= 1;
    }

    result
}

fn main() {
    println!("{}", title_to_number("FXSHRXW".to_string()));
}
