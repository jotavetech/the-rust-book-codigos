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
    let _s4 = s3; // válido

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




    


    // OWNERSHIP E FUNÇÕES

    let s7 = String::from("hello s7");  // s vem no escopo

    takes_ownership(s7); // s é passado para a função e não é mais válido fora
    // println!("{s7}"); -> s7 não é mais válido 

    let x = 5;  // x esta no escopo

    makes_copy(x); // x é passado para a função, mas como é do tipo i32, sabemos o tamanho e ele pode ficar, então continua sendo válido.

    println!("{x}"); // x continua válido







    // REFERENCIAS

    let string_nova = String::from("hello");

    let len = calculate_length(&string_nova);

    println!("The length of {} is {}", string_nova, len);


    let mut string_2 = String::from("hello");
    
    change(&mut string_2);

    println!("{string_2}"); // alterou e agora é hello, world
    
    let mut string_3 = String::from("hello");

    {
        let _r1 = &mut string_3;
    } // r1 sai do escopo e é droppado

    let _r2 = &mut string_3;

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);


}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn takes_ownership(some_string: String) { 
    println!("{}", some_string);
} // aqui, some_string sai do escopo e `drop` é chamado

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // aqui, some_integer sai do escopo. nada especial acontece

fn calculate_length(s: &String) -> usize {
    s.len()
}