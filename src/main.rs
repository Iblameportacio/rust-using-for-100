//Calcula el factorial de un número n dado.
use std::io;
fn main() {
    let mut resultado = 1;
    println!("escriba un numero");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("fallo, escribe un numero.");
    let n: i32 = n.trim().parse().expect("no es número");
    for i in 1..=n {
        resultado *= i;
    }
    println!("el factorial de {:?} es: {:?}", n, resultado)
}
