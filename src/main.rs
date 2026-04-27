//Imprime el cuadrado ($n^2$) de los números del 1 al 20.
fn main() {
    for i in 1..=20 {
        let inicio: i32 = i;
        let cuadrado = inicio.pow(2);
        println!("{}", cuadrado)
    }
}
