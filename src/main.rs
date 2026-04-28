//Imprime "Hola Rust" 50 veces, numerando cada línea.
fn main() {
    let hola = "hola rust";
    for i in 1..=50 {
        println!("{:?} {:?}", i, hola)
    }
}
