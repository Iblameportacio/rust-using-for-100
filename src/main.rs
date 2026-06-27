//Verifica si un número es abundante (suma divisores > n)
// Para saber si un número es abundante,
// primero tenemos que entender qué es un divisor propio.
// Un divisor propio de un número $n$ es cualquier número entero
// positivo más pequeño que él que lo divide exactamente
// (es decir, que el residuo de la división sea cero).
// si sumamos esos numero y el resultado es mayor a n
// entonces es un número abundante
fn main() {
    let numero = 12; //queremos saber si este número es abundante
    let mut es_abundante = false;

    let mut suma = 0;
    for i in 1..numero {
        if numero % i == 0 {
            suma += i;
        }
    }
    if suma > numero {
        es_abundante = true;
    }
    println!("el numero {} es abundante? {}", numero, es_abundante)
}
