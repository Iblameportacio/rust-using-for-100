//Cuenta cuántos números son múltiplos de 4 entre 1 y 200.
fn main() {
    let mut contador = 0;
    for i in 1..=200 {
        if i % 4 == 0 {
            contador += 1;
        }
    }
    println!("hay {} numeros multiplos de 4 entre uno y 200", contador)
}
