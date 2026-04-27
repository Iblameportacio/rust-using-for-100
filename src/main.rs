//Tabla de multiplicar del 5.
fn main(){
    let num: i32 = 5;
    for i in 0..=12{
        let solucion = num * i;
        println!("{:?} * {:?} = {:?}", num, i, solucion);
    }

}
