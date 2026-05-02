//Suma los divisores de un número.
fn main() {
    let a = 436;
    let mut inicio = 0;
    println!("los divisores de {} son:", a);
    for i in 1..=a {
        if a % i == 0 {
            println!("{}", i);
            inicio += i;
        }
    }
    print!("y su suma es: {}", inicio);
}
