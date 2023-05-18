use std::hash::Hasher;


pub fn tokenize(text : &str, rules : &Vec<&str>) -> Vec<(usize, String)> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for (position, character) in text.chars().enumerate() {
        if rules.contains(&character.to_string().as_str()) {
            if !current_token.is_empty() {
                tokens.push((position - current_token.len(), current_token));
                current_token = String::new();
            }

        } else {
            current_token.push(character);
        }
    }

    if !current_token.is_empty() {
        tokens.push((text.len() - current_token.len(), current_token));
    }
    tokens
}

const ALLOCABLE_SIZE: usize = 100;

#[derive(Debug)]
pub struct Entry<K, V> {
    key: K,
    value: V,
}

//add display
#[derive(Debug)]
pub struct HashTable<K, V> {
    table: Vec<Option<Entry<K, V>>>,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + std::hash::Hash,
{
    pub fn new() -> Self {
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

    pub fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        self.table[index] = Some(Entry { key, value });
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.hash(key);
        match &self.table[index] {
            Some(entry) if entry.key == *key => Some(&entry.value),
            _ => None,
        }
    }

    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let index = self.hash(&key);
        match &mut self.table[index] {
            Some(entry) if entry.key == *key => Some(&mut entry.value),
            _ => None,
        }
    }
}