pub fn inspect(s: &String) {
    println!("{:?}", s.ends_with("s"));
}

pub fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

pub fn eat(s: String) -> bool {
    s.starts_with('b') && s.contains('a')
}

pub fn add(x: &i32, y: &i32) -> i32 {
    *x + *y
}