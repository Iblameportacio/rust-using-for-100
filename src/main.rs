//Encuentra la posición (índice) de un elemento en un vector.
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let buscar_este_numero = 6;
    for i in 0..v.len() {
        if v[i] == buscar_este_numero {
            println!(
                "el numero {} esta en la posicion v[{}]",
                buscar_este_numero, i
            )
        }
    }
}
