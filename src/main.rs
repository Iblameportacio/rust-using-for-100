//Crea un vector que contenga los cuadrados de otro vector.
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let vec_rev_iter: Vec<_> = v.iter().map(|x| x * x).collect();
    for i in vec_rev_iter {
        println!("{}", i);
    }
}
