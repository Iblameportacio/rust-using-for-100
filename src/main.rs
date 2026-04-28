//Calcula el producto de los números del 1 al 10.
fn main() {
    let mut resultado = 1;
    for i in 1..=10 {
        resultado *= i;
    }
    println!("{}", resultado);
}
