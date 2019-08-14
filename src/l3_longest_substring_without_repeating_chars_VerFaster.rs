use std::collections::HashMap;
use std::cmp::max;

fn length_of_longest_substring(s: String) -> i32 {
       let mut hash_map = HashMap::with_capacity(s.len());
        let mut answer = 0;
        let mut idx_s = 0;

        for (idx, item) in s.chars().enumerate() {
            match hash_map.get(&item) {
                Some(collision_idx) => {
                    idx_s = max(*collision_idx+1, idx_s);},
                None => {}
            }
            answer = max(answer, idx+1 - idx_s);
            hash_map.insert(item, idx);
        }
        answer as i32
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