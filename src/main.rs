//Suma todos los números primos entre 1 y 50.
fn main() {
    let mut es_primo: bool;
    for i in 1..=50 {
        es_primo = true;
        for j in 2..i {
            if i % j == 0 {
                es_primo = false;
                break;
            }
        }
        if i > 1 && es_primo {
            println!("{}", i);
        }
    }
}
