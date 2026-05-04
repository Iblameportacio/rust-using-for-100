//Calcula el promedio de los valores de un vector de f32.
fn main() {
    let v = vec![
        1.0, 10.3, 2.3, 1.44, 4.1, 2.32, 3.0, 6.7, 5.7, 3.5, 0.2, 5.6, 9.8, 82.79, 19.8, 67.0, 1.2,
        2.1, 3.4, 6.9,
    ];
    let mut inicio: f32 = 0.0;
    let total = v.len();
    for i in v {
        //im not gonna use &v because im not going use the vector again in this problem
        inicio += i;
    }
    let prom = inicio / total as f32;
    println!("{}", prom);
}
