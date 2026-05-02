//Calcula la potencia $a^b$ usando solo bucles y multiplicaciones (sin .pow()).
fn main() {
    let a = 3;
    let b = 4;
    let mut resultado = 1;

    for _i in 1..=b {
        resultado *= a;
    }

    println!("{} elevado a {} es {}", a, b, resultado);
}
