//Invierte un vector manualmente usando un bucle.
fn main() {
    let v = vec![
        1, 2, 3, 4, 6, 32, 4, 667, 9, 8, 8236, 3132, 5, 451, 654, 613341, 1, 1, 1, 1, 1,
    ];
    let vec_rev_iter: Vec<_> = v.iter().rev().collect();
    for i in vec_rev_iter {
        println!("{}", i);
    }
}
