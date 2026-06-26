//Tabla de multiplicar del 1 al 10 con formato alineado (usa {:>3})
// im back i guess
fn main() {
    for i in 1..=10 {
        println!("esta es la tabla del: {}", i);
        for z in 1..=10 {
            let solved = z * i;
            println!("{:>3} * {:>3} = {:>3}", i, z, solved);
        }
        println!(""); //print vacío para un salto entre las líneas
    }
}
