pub fn multiply(num1: String, num2: String) -> String {
    let mut product = vec![0; num1.len() + num2.len()];
    let mut offset = 0;

    for c1 in num1.chars().map(|c| c.to_digit(10).unwrap()).rev() {
        let mut carry = 0;

        for (index, c2) in num2
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .rev()
            .enumerate()
        {
            let p = c1 * c2 + carry;
            carry = p / 10;

            product[index + offset] += p % 10;
        }

        if carry > 0 {
            product[num2.len() + offset] += carry;
        }

        offset += 1;
    }

    for index in 0..product.len() {
        let carry = product[index] / 10;
        product[index] = product[index] % 10;

        if carry > 0 {
            product[index + 1] += carry;
        }
    }

    let char_array = product.iter().rev().skip_while(|&x| *x == 0);

    if char_array.clone().collect::<Vec<&u32>>().len() == 0 {
        return "0".to_string();
    }

    char_array
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn main() {
    println!("{}", multiply("123".to_string(), "456".to_string()));
}
