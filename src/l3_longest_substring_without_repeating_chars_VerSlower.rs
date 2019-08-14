use std::collections::HashMap;
use std::cmp;

fn length_of_longest_substring(s: String) -> i32 {
    let mut max_len = 0;
    let mut chars_map = HashMap::new();

    for i in s.char_indices() {
        chars_map.entry(i.1).or_insert(i.1);
        for j in s.char_indices() {
            if j.0 < i.0 {
                continue;
            } else if j.0 == i.0 {
                max_len = cmp::max(max_len, (j.0 as i32 - i.0 as i32 + 1));
                continue;
            }
            if chars_map.get(&j.1) == None {
                chars_map.entry(j.1).or_insert(j.1);
                max_len = cmp::max(max_len, (j.0 as i32 - i.0 as i32 + 1));
                continue;
            } 
            break;
        }
        chars_map.clear();
    }
    max_len
}

pub fn run () {
    let test_strs = String::from("abcabcbb");
    println!("{:?}", length_of_longest_substring(test_strs));
    let test_strs_1 = String::from("bbbbb");
    println!("{:?}", length_of_longest_substring(test_strs_1));
    //
    // Expected len = 1 
    //
    let test_strs_1 = String::from(" ");
    println!("{:?}", length_of_longest_substring(test_strs_1));
    //
    // Expected len = 0
    //
    let test_strs_1 = String::from("");
    println!("{:?}", length_of_longest_substring(test_strs_1));
    let test_strs_1 = String::from("dvdf");
    println!("{:?}", length_of_longest_substring(test_strs_1));
}