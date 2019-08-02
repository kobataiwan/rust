extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
    let re = Regex::new(r"[^a-z]").unwrap();
    let lower = paragraph.to_lowercase();
    let pure  = re.replace_all(&lower," ");
    let words = pure.split_whitespace();
    let mut word_cnt = HashMap::new();
    let banned:HashSet<String> = banned.into_iter().collect();

    for word in words {
        if banned.contains(word) {
            continue;
        }
        *word_cnt.entry(word).or_insert(0) += 1;
    }
    let mut max_str: String = String::from("");
    let mut max_val: i32 = 0;
    for node in word_cnt.iter() {
        if max_val < *node.1 {
            max_val = *node.1;
            max_str = node.0.to_string();
        }
    }
    max_str
}

pub fn run () {
    let test = String::from("this is a test string is a TEST! TeSt,");
    let test_ban = vec!["is".to_string()];
    println!("{:?}", most_common_word(test, test_ban)); 
}
