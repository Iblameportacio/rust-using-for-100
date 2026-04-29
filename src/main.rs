//Imprime los primeros 20 números de la serie de Fibonacci.

fn main() {
    let mut a = 0;
    let mut b = 1;
    for _i in 0..20 {
        let i = a + b;
        let solucion = a;
        println!("{:?}", solucion);
        a = b;
        b = i;
    }
}
