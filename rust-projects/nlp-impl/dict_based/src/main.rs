use substring::Substring;

fn main() {
        let text : String = "hello world.hello world hello world".to_string();
        let rules = vec![" ", ".", ","];
        let dict_tokens : Vec<String> = vec!["world.hello".to_string()];
    
        let mut tokens = Vec::new();
        let mut current_token = String::new();

        let mut dict_loop = 0;
    
        'main: for (position, character) in text.chars().enumerate() {
            //looking for the dictionary
            if dict_loop > 0{
                dict_loop -= 1;
                continue;
            }

            if rules.contains(&character.to_string().as_str()) {
                if !current_token.is_empty() {
                    //validate if the token is in the dictionary
                    for dict_token in dict_tokens.iter() {
                        if (**dict_token).to_string().substring(0, current_token.len()) == current_token {
                            if dict_token == text
                                .to_string()
                                .substring(
                                    position - current_token.len(), 
                                    position - current_token.len() + dict_token.len()) 
                            {
                                tokens.push(String::from(dict_token.clone()));
                                dict_loop = dict_token.len() - current_token.len();
                                current_token = String::new();
                                continue 'main;
                            }
                        }
                    }
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
