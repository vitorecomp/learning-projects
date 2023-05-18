use std::hash::Hasher;
use substring::Substring;

const ALLOCABLE_SIZE: usize = 100;

#[derive(Debug)]
struct Entry<K, V> {
    key: K,
    value: V,
}

//add display
#[derive(Debug)]
struct HashTable<K, V> {
    table: Vec<Option<Entry<K, V>>>,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new() -> Self {
        let mut table = HashTable {
            table: vec![],
        };
        for _ in 0..ALLOCABLE_SIZE {
            table.table.push(None);
        }
        return table;
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        let pos = hasher.finish() as usize % ALLOCABLE_SIZE;
        pos
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        self.table[index] = Some(Entry { key, value });
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        match &self.table[index] {
            Some(entry) if entry.key == *key => Some(&entry.value),
            _ => None,
        }
    }
}

fn main() {

    let text : String = "hello world,hello world hello world. second sentence".to_string();
    let rules = vec![" ", ".", ","];
    let sentence_rules = vec!["."];
    let dict_tokens : Vec<String> = vec!["world,hello".to_string()];

    let mut tokens = Vec::new();
    let mut current_token = String::new();

    let mut dict_loop = 0;
    let mut sentence = 0;

    'main: for (position, character) in text.chars().enumerate() {
        //looking for the dictionary
        if dict_loop > 0{
            dict_loop -= 1;
            continue;
        }

        if rules.contains(&character.to_string().as_str()) || sentence_rules.contains(&character.to_string().as_str()) {

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
                            tokens.push(((position - current_token.len(), sentence), String::from(dict_token.clone())));
                            dict_loop = dict_token.len() - current_token.len();
                            current_token = String::new();
                            continue 'main;
                        }
                    }
                }
                tokens.push(((position - current_token.len(), sentence), current_token));
                current_token = String::new();
            }
            if sentence_rules.contains(&character.to_string().as_str()) {
                sentence += 1;
            }
        } else {
            current_token.push(character);
        }
    }

    if !current_token.is_empty() {
        tokens.push(((text.len() - current_token.len(), sentence), current_token));
    }

    dbg!(&tokens);
    let mut index_hash_table :HashTable<String, Vec<(usize, u32)>> = HashTable::new();
    for token_tuple in tokens {
        let token = token_tuple.1;
        let position = token_tuple.0;

        if let Some(positions) = index_hash_table.get(&String::from(token.clone())) {
            let mut positions = positions.clone();
            positions.push(position);
            index_hash_table.insert(String::from(token.clone()), positions);
        }else{
            index_hash_table.insert(String::from(token.clone()), vec![position]);
        }
    }
    dbg!(&index_hash_table);
    dbg!(index_hash_table.get(&String::from("hello")));
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