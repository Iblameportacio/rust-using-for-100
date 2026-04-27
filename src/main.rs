//Tabla de multiplicar de un número ingresado por el usuario.
use std::io;
fn main() {
    println!("Por favor, introduce un numero:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("error");

    let num: i32 = input.trim().parse().expect("no es número");
    for i in 0..=12 {
        let solucion = num * i;
        println!("{} * {} = {}", num, i, solucion);
    }
}
