use hash_table_dict::{tokenize, HashTable};
use substring::Substring;



fn main() {

    let text : &str = "hello world.hello world hello world";
    let rules: Vec<&str> = vec![" ", ".", ","];
    let dict_tokens : Vec<&str> = vec!["world.hello"];

    
    let tokens = tokenize(text, &rules);

    let mut index_hash_table :HashTable<String, Vec<usize>> = HashTable::new();
    for (position, token) in tokens {
        match index_hash_table.get_mut(&token) {
            Some(positions) => positions.push(position),
            None => index_hash_table.insert(String::from(token), vec![position])
        }
    }
    dbg!(index_hash_table.get(&String::from("world")));
    
    let mut to_remove : Vec<(String, usize)> = vec![];

    for dict_token in dict_tokens {
        let dict_token_tokens = tokenize(dict_token, &rules);
        if let Some (positions) = index_hash_table.get_mut(&dict_token_tokens[0].1) {
            for (_, position) in positions.to_vec().iter().enumerate() {
                if dict_token == text.substring(*position, *position + dict_token.len()) {
                    index_hash_table.insert(String::from(dict_token), vec![*position]);
                    
                    //todo do remove all the tokens other tokens
                    to_remove.push((dict_token_tokens[0].1.to_string(), *position));
                    let mut moving_position = *position + 1 + dict_token_tokens[0].1.len();
                    for i in 1..dict_token_tokens.len(){
                        to_remove.push((dict_token_tokens[i].1.to_string(), moving_position));
                        moving_position += dict_token_tokens[i].1.len() + 1;
                    }
                }
            }
        } 
    }

    dbg!(&to_remove);

    for remove in to_remove {
        if let Some(positions) = index_hash_table.get_mut(&remove.0) {
            positions.remove(positions.iter().position(|&r| r == remove.1).unwrap());
        }
    }
    
    //todo remove the other tokens
    dbg!(&index_hash_table);
    dbg!(index_hash_table.get(&String::from("world.hello")));
}


// let mut index_hash_table :HashTable<String, Vec<usize>> = HashTable::new();

// let tokens = vec!["hello", "world", "hello", "world", "hello", "world"];

// for (position, token) in tokens.iter().enumerate() {
//     if let Some(positions) = index_hash_table.get(&String::from(token.clone())) {
//         let mut positions = positions.clone();
//         positions.push(position);
//         index_hash_table.insert(String::from(token.clone()), positions);
//     }else{
//         index_hash_table.insert(String::from(token.clone()), vec![position]);
//     }
// }

// dbg!(index_hash_table.get(&String::from("hello")));