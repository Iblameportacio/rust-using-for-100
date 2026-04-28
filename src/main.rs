//Imprime los números del 1 al 100, pero sáltate el 13 y el 66.
fn main() {
    for i in 1..=100 {
        if i == 13 || i == 66 {
            continue;
        }
        println!("{}", i)
    }
}
