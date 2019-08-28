fn reverse_string(s: &mut Vec<char>) {
    let s_len = s.len();
    let vec_len = s_len / 2;
    for idx in 0..vec_len {
        let tmp = s[idx];
        s[idx] = s[s_len - idx - 1];;
        s[s_len - idx - 1] = tmp;
    }
}

fn main() {
    let mut test_str = vec!['h','e','l','l','o'];
    reverse_string(& mut test_str);
    println!("reverse str {:?}", test_str);
}