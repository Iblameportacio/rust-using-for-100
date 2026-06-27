//Números de Tribonacci (como Fibonacci pero suma los 3 anteriores)
fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    for _i in 0..10 {
        let i = a + b + c;
        let solucion = a;
        println!("{:?}", solucion);
        a = b;
        b = c;
        c = i;
    }
}
