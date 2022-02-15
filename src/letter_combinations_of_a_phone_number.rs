use std::{array::IntoIter, collections::HashMap, vec};

pub fn letter_combinations(digits: String) -> Vec<String> {
    let digits_map = HashMap::<u8, &str>::from_iter(IntoIter::new([
        (2, "abc"),
        (3, "def"),
        (4, "ghi"),
        (5, "jkl"),
        (6, "mno"),
        (7, "pqrs"),
        (8, "tuv"),
        (9, "wxyz"),
    ]));
    let mut result = if digits.len() != 0 {
        vec!["".to_string()]
    } else {
        vec![]
    };

    for digit in digits.as_bytes() {
        let mut aux: Vec<String> = vec![];

        for char in digits_map[&(digit - 48)].as_bytes() {
            for string in &result {
                let mut new_string = String::from(string);
                new_string.push(*char as char);
                aux.push(new_string);
            }
        }

        result = aux;
    }

    result
}

fn main() {
    println!("{:?}", letter_combinations("23".to_string()));
}
