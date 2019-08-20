use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut s_cnt = 0;
    let mut s_norm: Vec::<i32> = Vec::new();
    let mut t_norm: Vec::<i32> = Vec::new();
    let mut str_map = HashMap::new();

    for c in s.chars() {
        match str_map.get(&c) {
            Some(idx) =>  { 
                println!("c {:?}, idx {:?}", c, idx);
                s_norm.push(*idx);
            },
            None => {
                println!("{:?}", c);
                s_cnt += 1;
                str_map.entry(c).or_insert(s_cnt);
                s_norm.push(s_cnt);
            }
        }
    }
    str_map.clear();
    s_cnt = 0;
    println!("{:?}", s_norm);
    for c in t.chars() {
        match str_map.get(&c) {
            Some(idx) =>  { 
                t_norm.push(*idx);
            },
            None => {
                println!("{:?}", c);
                s_cnt += 1;
                str_map.entry(c).or_insert(s_cnt);
                t_norm.push(s_cnt);
            }
        }
    }
    println!("{:?}", t_norm);
    if t_norm.len() == t_norm.iter().zip(s_norm.iter()).filter(|&(a,b)| a==b).count() {
        true
    } else {
        false
    }
}

pub fn run () {
    let test_str1 = String::from("egg");
    let test_str2 = String::from("add");
    println!("is iso? {:?}", is_isomorphic(test_str1, test_str2));
    let test_str1 = String::from("foo");
    let test_str2 = String::from("bar");
    println!("is iso? {:?}", is_isomorphic(test_str1, test_str2));
    let test_str1 = String::from("paper");
    let test_str2 = String::from("title");
    println!("is iso? {:?}", is_isomorphic(test_str1, test_str2));
    let test_str1 = String::from("aba");
    let test_str2 = String::from("baa");
    println!("is iso? {:?}", is_isomorphic(test_str1, test_str2));
}