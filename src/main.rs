//Crea un vector del 1 al 10 y súmalo.
fn main() {
    let mut inicio = 0;
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in v {
        inicio += i;
    }
    println!("la suma total del vector es: {}", inicio);
}
