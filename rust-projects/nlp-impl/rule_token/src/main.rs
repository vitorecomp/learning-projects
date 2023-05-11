fn main() {
    let text = "hello world.hello world hello world";
    let rules = vec![" ", ".", ","];

    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for character in text.chars() {
        if rules.contains(&character.to_string().as_str()) {
            if !current_token.is_empty() {
                tokens.push(current_token);
                current_token = String::new();
            }
        } else {
            current_token.push(character);
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    dbg!(tokens);
}
