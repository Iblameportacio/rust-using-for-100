//Imprime los primeros 4 números perfectos.
fn main() {
    println!(
        "Un número perfecto es aquel que es igual a la
        suma de sus divisores propios (excluyéndose a sí mismo)"
    );
    for z in 1..=10000 {
        let mut suma = 0;
        for n in 1..z {
            if z % n == 0 {
                suma += n;
            }
        }
        if suma == z {
            println!("{}", z);
        }
    }
}
