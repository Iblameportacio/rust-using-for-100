//Calcula la suma de una progresión aritmética ($a, a+d, a+2d...$) de 10 términos.
fn main() {
    let a = 2;
    let d = 3;
    let mut contador = 0;
    for i in 0..10 {
        let cumulus = a + i * d;
        contador += cumulus;
    }
    println!("{}", contador)
}
