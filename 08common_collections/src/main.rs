use std::collections::HashMap;
use std::fmt::format;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut w = vec![1, 2, 3];

    v.push(1);
    w.push(4);

    println!("w: {:?}", w);
    println!("v: {:?}", v);

    let c = vec![1, 2, 3, 4, 5];

    let first = &c[0];

    // c.push(6); // mudou o lugar do vetor na memória e agora não é mais valido.

    println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("v[0]: {}", v[0]);

    // Strings UTF-8

    let _s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("foo {s}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{s1}-{s2}-{s3}");

    println!("{s}");

    let slice = "string 1";
    let part = &slice[0..2];

    println!("{}", part);

    for c in "Зд".chars() {
        println!("{c}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{score}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
