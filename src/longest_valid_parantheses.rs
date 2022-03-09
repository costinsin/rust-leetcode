use std::vec;

pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut index_vec = vec![];
    for i in 0..s.len() as i32 - 1 {
        let i  = i as usize;

        if &s[i..i + 2] == "()" {
            index_vec.push((i, i + 1));
        }
    }

    let mut can_merge = true;

    println!("{:?}", index_vec);

    while can_merge && index_vec.len() > 0 {
        can_merge = false;

        // Try to merge consecutive groups
        let mut i = 0;
        while i < index_vec.len() - 1 {
            if index_vec[i].1 + 1 == index_vec[i + 1].0 {
                let aux = index_vec[i + 1];

                index_vec.remove(i + 1);
                index_vec[i] = (index_vec[i].0, aux.1);
                can_merge = true;
            } else {
                i += 1;
            }
        }

        println!("After merge: {:?}", index_vec);

        // Try to extend each group
        let mut i = 0;
        while i < index_vec.len() {
            if index_vec[i].0 > 0
                && index_vec[i].1 < s.len() - 1
                && s.as_bytes()[index_vec[i].0 - 1] == '(' as u8
                && s.as_bytes()[index_vec[i].1 + 1] == ')' as u8
            {
                index_vec[i] = (index_vec[i].0 - 1, index_vec[i].1 + 1);
                can_merge = true;
            }
            i += 1;
        }

        println!("After extension: {:?}", index_vec);
    }

    index_vec
        .into_iter()
        .map(|x| x.1 - x.0 + 1)
        .max()
        .unwrap_or(0) as i32
}

fn main() {
    println!("{}", longest_valid_parentheses("()((()(())((()))())".to_string()));
}
