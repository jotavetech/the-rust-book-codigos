fn main() {
    let mut v: Vec<i32> = Vec::new();
    let mut w = vec![1, 2, 3];

    v.push(1);
    w.push(4);

    println!("w: {:?}", w);
    println!("v: {:?}", v);

    let mut c = vec![1, 2, 3, 4, 5];

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
}
