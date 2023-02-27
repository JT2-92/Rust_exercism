pub fn reverse(input: &str) -> String {
    let mut chars: Vec<char> = input.chars().collect();
    let mut i = 0;
    let mut j = chars.len() - 1;
    while i < j {
        chars.swap(i, j);
        i += 1;
        j -= 1;
    }
    chars.into_iter().collect()
}

pub fn reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let mut reversed = String::new();
    let mut len = chars.len();
    if len > 1 {
        let mut idx = chars.len() - 1;
        loop {
            reversed.push(chars[idx]);
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
    };
    reversed
}
