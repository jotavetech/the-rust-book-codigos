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

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let minha_array = [1, 2, 3, 4];

    // arrays, tem tamanho fixo:
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    println!("{}", months[11]);
}
