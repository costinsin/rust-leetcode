fn check_parenthesis(stack: &mut Vec<char>, parenthesis: char) -> Option<bool> {
    if let Some(x) = stack.pop() {
        if x != parenthesis {
            return Some(false);
        }
    } else {
        return Some(false);
    }
    None
}

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.as_bytes() {
        match *c as char {
            '(' | '[' | '{' => {
                stack.push(*c as char);
            }
            ')' => {
                if let Some(value) = check_parenthesis(&mut stack, '(') {
                    return value;
                }
            }
            ']' => {
                if let Some(value) = check_parenthesis(&mut stack, '[') {
                    return value;
                }
            }
            '}' => {
                if let Some(value) = check_parenthesis(&mut stack, '{') {
                    return value;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", is_valid("()[]{}{".to_string()));
}
