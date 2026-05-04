//Encuentra el número mayor en un vector de 20 números aleatorios.
fn main() {
    let v = vec![
        1, 10, 23, 14, 4, 2, 3, 6, 57, 35, 2, 56, 9, 8279, 98, 670, 12, 21, 34, 69,
    ];
    let mut mayor = v[0];
    for i in v {
        if mayor < i {
            mayor = i;
        }
    }
    println!("{}", mayor)
}
