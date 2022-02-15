pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs[0].as_str();

    for i in 1..strs.len() {
        for j in 0..prefix.len() {
            if j >= strs[i].len() || prefix.as_bytes()[j] != strs[i].as_bytes()[j] {
                prefix = &prefix[..j];
                break;
            }
        }
    }

    prefix.to_string()
}

fn main() {
    let vec = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    println!("{}", longest_common_prefix(vec));
}
