//Crea una función que reciba un número n y
// devuelva el producto (la multiplicación) de los cubos (potencia de 3)
// de todos los números pares entre 1 y n
fn main() {
    let n: u16 = 3;
    println!("n es {}", n);
    println!("y su solucion es {}", solucion(n))
}
fn solucion(n: u16) -> u16 {
    let n: u16 = (1..=n).filter(|n| n % 2 == 0).map(|n| n.pow(3)).product();
    n
}
