//Imprime los múltiplos de 3 entre 1 y 90.
fn main() {
    for i in 1..=90 {
        if i % 3 == 0 {
            println!("{}", i)
        }
    }
}
