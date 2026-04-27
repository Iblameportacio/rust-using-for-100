//Imprime el cubo ($n^3$) de los números del 1 al 15.
fn main() {
    for i in 1..=15 {
        let inicio: i32 = i;
        let cuadrado = inicio.pow(3);
        println!("{}", cuadrado)
    }
}
