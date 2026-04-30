//Imprime los números que son divisibles por 3 y 5 al mismo tiempo (1 al 150).
fn main() {
    for i in 1..=150 {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{}", i)
        }
    }
}
