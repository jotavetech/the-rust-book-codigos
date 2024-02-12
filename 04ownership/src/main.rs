fn main() {
    {
        let _s = "hello";
    }

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() coloca uma string literal

    println!("{}", s); //  hello, world!

    let s1 = String::from("hello");
    let s2 = s1; // agora s1 é invalido, o novo dono é s2

    // println!("{s1}"); -> da erro
    println!("{s2}");

    let s3 = "teste"; // válido
    let s4 = s3; // válido

    let s5 = String::from("e agora hein");

    {
        let s6 = s5;
        // println!("{s5}"); -> inválido, s6 agora é o dono e s5 deixa de existir
        println!("{s6}");
    }

    let num1 = 5;
    let num2 = num1;

    // ambas funcionam pois um inteiro é um valor que sabemos o tamanho e fica na memória stack
    println!("{num1}"); 
    println!("{num2}");


    // ownership e funções

    let s7 = String::from("hello s7");  // s vem no escopo

    takes_ownership(s7); // s é passado para a função e não é mais válido fora
    // println!("{s7}"); -> s7 não é mais válido 

    let x = 5;  // x esta no escopo

    makes_copy(x); // x é passado para a função, mas como é do tipo i32, sabemos o tamanho e ele pode ficar, então continua sendo válido.

    println!("{x}"); // x continua válido
}

fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} // aqui, some_string sai do escopo e `drop` é chamado

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // aqui, some_integer sai do escopo. nada especial acontece
