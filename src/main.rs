//Suma de la serie 1 + 1/2 + 1/3 + ... + 1/n
fn main() {
    let n = 25;
    let mut suma: f64 = 1.0;
    for i in 2..=n {
        suma = suma + 1.0 / (i as f64);
    }
    println!("{}", suma)
}
