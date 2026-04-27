//Imprime los múltiplos de 5 entre 1 y 100 en orden descendente.
fn main() {
    for i in (1..=100).rev() {
        if i % 5 == 0 {
            println!("{}", i)
        }
    }
}
