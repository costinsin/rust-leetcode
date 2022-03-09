pub fn str_str(haystack: String, needle: String) -> i32 {
    for i in 0..haystack.len() as i32 - needle.len() as i32 + 1 {
        if &needle[..] == &haystack[i as usize..i as usize + needle.len()] {
            return i as i32;
        }
    }

    -1
}

fn main() {
    println!("{}", str_str("abb".to_string(), "abaaa".to_string()));
}
