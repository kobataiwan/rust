use std::collections::HashMap;
use std::collections::HashSet;

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let words = paragraph.split_whitespace();
    let mut word_cnt = HashMap::new();
    let banned:HashSet<String> = banned.into_iter().collect();

    for word in words {
        if banned.contains(word) {
            continue;
        }
        *word_cnt.entry(word).or_insert(0) += 1;
    }

    let rst = match word_cnt.iter().max_by_key(|&(_key, val)| val) {
        Some(x) => Some(x).into_result()(),
        None => (None,0),
    };
    paragraph
}

fn main() {
    let test = String::from("this is a test string is a");
    let test_ban = vec!["is".to_string()];
    most_common_word(test, test_ban); 
}
