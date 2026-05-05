//Suma dos vectores elemento por elemento (deben tener el mismo tamaño).
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let w = vec![2, 3, 4, 5, 6, 7];
    let mut solucion = Vec::new(); // Creamos el nuevo vector vacío
    for i in 0..v.len() {
        solucion.push(v[i] + w[i]);
    }
    println!("{:?}", solucion);
}
