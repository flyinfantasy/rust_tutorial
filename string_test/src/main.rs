fn main() {
    let s1 = String::from("foo");
    //s.push_str("bar");
    let s2 = String::from("bar");
    let s = format!("{s1}{s2}");
    println!("Test is {s}");
    let f = &s[0..1];
    println!("Test is {f}");
    for c in s.chars(){
        println!("{c}");
    }
}
