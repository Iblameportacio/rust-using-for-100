//Imprime el factorial de 7
fn main() {
    let mut inicio = 1;
    for i in 1..=7 {
        inicio *= i;
    }
    println!("el factorial de 7 es: {}", inicio)
}
