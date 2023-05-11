fn main() {
    let text = "hello world.hello world hello world";

    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for character in text.chars() {
        if character.is_whitespace() {
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
