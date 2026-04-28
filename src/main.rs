//Imprime los números de 5 en 5 hasta el 250.
fn main() {
    for i in (5..=250).step_by(5) {
        println!("{}", i)
    }
}
