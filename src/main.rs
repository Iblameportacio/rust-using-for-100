//Calcula la media aritmética de los números del 1 al n
fn main() {
    let n = 10;
    let mut suma = 0;
    for i in 1..=n {
        suma += i;
    }
    println!("la suma es: {}", suma);
    let media_aritmetica: f32 = suma as f32 / n as f32;
    println!("la media aritmetica es: {}", media_aritmetica);
}
