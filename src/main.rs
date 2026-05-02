// Imprime la tabla de multiplicar del 1 al 10 (tablas anidadas).
fn main() {
    for i in 1..=10 {
        for z in 1..=10 {
            let solved = i * z;
            println!("{:?}*{:?}={:?}", i, z, solved);
        }
    }
}
