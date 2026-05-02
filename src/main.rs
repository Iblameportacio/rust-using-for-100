// Imprime los números del 1 al 100,
//  pero sustituye los múltiplos de 4 por la palabra "Cuatro".
fn main() {
    for i in 1..=100 {
        if i % 4 == 0 {
            println!("cuatro");
        } else {
            println!("{}", i);
        }
    }
}
