//Encuentra el número menor en un vector.
fn main() {
    let v = vec![
        1, 10, 23, 14, 4, 2, 3, 6, 57, 35, 2, 56, 9, 8279, 98, 670, 12, 21, 34, 69,
    ];
    let mut menor = v[0];
    for i in v {
        if menor > i {
            menor = i;
        }
    }
    println!("{}", menor);
}
