//Cuenta cuántos divisores tiene un número.
fn main() {
    let a = 436;
    let mut contador = 0;
    for i in 1..=a {
        if a % i == 0 {
            contador += 1;
        }
    }
    println!("el numero {} tiene {} divisores", a, contador)
}
