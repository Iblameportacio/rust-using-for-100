//Verifica si un número n es primo (usando un bucle para probar divisores).
fn main() {
    let numero = 4;
    let mut es_primo: bool = true;
    for i in 2..=numero - 1 {
        if numero % i == 0 {
            es_primo = false;
        }
    }
    println!("el numero {} es primo? {}", numero, es_primo);
}
