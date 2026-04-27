//Suma de todos los números pares entre 1 y 100.
fn main() {
    let mut inicio = 0;
    for i in 1..=100 {
        if i % 2 == 0 {
            inicio = inicio + i;
        }
    }
    println!("{}", inicio)
}
