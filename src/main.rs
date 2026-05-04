//Cuenta cuántas veces aparece un número específico en un vector.
fn main() {
    let v = vec![
        1, 2, 3, 4, 6, 32, 4, 667, 9, 8, 8236, 3132, 5, 451, 654, 613341, 1, 1, 1, 1, 1,
    ];
    let mut contador = 0;
    let numero = 1;
    for i in v {
        //im not gonna use &v because im not going use the vector again in this problem
        if numero == i {
            contador += 1;
        }
    }
    println!("hay {} unos en el vector ", contador);
}
