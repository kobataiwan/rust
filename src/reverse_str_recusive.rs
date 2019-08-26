fn reverse_str(s:String)-> String {
    let mut rvrs_s = String::new();
    if 1 == s.chars().count() {
        s.get(0..s.len()).unwrap().to_string()
    } else { 
        rvrs_s.push_str(reverse_str(
            match s.get(1..s.len()) {
                Some(x) => x.to_string(),
                None => String::from("")
            }).as_mut_str());
        rvrs_s.push_str(
            match s.get(0..1) {
                Some(x) => x.to_string(),
                None => String::from("")
            }.as_mut_str());
        rvrs_s
    }
}

fn main() {
    let test_str = String::from("abcdefghijklmnopqrstuvwxyz");
    println!("Reverse {:?}", reverse_str(test_str));
}