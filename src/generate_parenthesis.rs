pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn helper(n: i32, open: i32, current_strings: Vec<String>) -> Vec<String> {
        if n == 0 {
            return current_strings;
        }

        let mut open_strings  = vec![];
        let mut close_strings = vec![];

        // If one '(' can be added
        if open < n {
            let mut updated_strings = current_strings.clone();
            updated_strings.iter_mut().for_each(|string| string.push('('));

            open_strings = helper(n, open + 1, updated_strings);
        }

        // If one ')' can be added
        if open > 0 {
            let mut updated_strings = current_strings.clone();
            updated_strings.iter_mut().for_each(|string| string.push(')'));

            close_strings = helper(n - 1, open - 1, updated_strings);
        }

        open_strings.append(&mut close_strings);

        open_strings
    }

    helper(n, 0, vec!["".to_string()])
}

fn main() {
    println!("{:?}", generate_parenthesis(3));
}
