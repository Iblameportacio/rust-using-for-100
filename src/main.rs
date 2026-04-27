//Suma de los primeros 50 números naturales.
fn main() {
    let mut inicio = 0;
    for i in 1..=50 {
        inicio = inicio + i;
    }
    println!("{}", inicio)
}
