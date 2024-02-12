fn main() {
    // Variables, constants & shadowing
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Shadowing
    let x = 10;
    let x = "X is a string";
    let x = true;

    println!("{x}");

    // data types

    //  escalaveis

    let inteiro: u32 = 100;
    let flutuante: f64 = 5.53;

    let resultado: f64 = inteiro as f64 - flutuante;

    println!("{resultado}");

    // compostos

    // tuplas

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (tup_um, dois, tres) = tup;
    println!("{tup_um}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;

    let _six_point_four = x.1;

    let _one = x.2;

    let _minha_array = [1, 2, 3, 4];

    // arrays, tem tamanho fixo:
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    println!("{}", months[11]);

    // funções

    another_function();

    another_function2(x.0);

    // controle de fluxo

    let is_raining = false;

    if is_raining {
        println!("Fique em casa!");
    } else {
        println!("Vá passear");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("O valor de result é: {result}");

    // while loops

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Saiu do loop!!!");

    // for loops

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..5) {
        println!("{number}!");
    }
    println!("Saiu do loop!!!");
}

fn another_function() {
    println!("Another function");
}

fn another_function2(x: i32) {
    println!("{x}");
}