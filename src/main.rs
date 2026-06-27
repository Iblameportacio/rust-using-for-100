//Cuenta regresiva de n hasta 0 con loop (no for)
fn main() {
    let mut n = 20;
    loop {
        println!("{}", n);
        n -= 1;
        if n < 0 {
            break;
        }
    }
}
