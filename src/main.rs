//Collatz: cuenta los pasos para llegar a 1 desde n
// ESTO SI QUE ES EMOCIONANTE, LA CONJETURA DE COLLATZ JAJAJAJJAJAJA
// La regla dice que si eliges cualquier número entero positivo y
// sigues un juego de dos reglas simples, tarde o temprano siempre vas a terminar llegando al número 1
// la ecuación es 3n+1 si el numero es impar lo multiplicas por tres y le sumas uno
// si es par lo divides por dos y así
fn main() {
    let mut numerito: i32 = 43;
    println!("para que el numero {} sea uno", numerito);
    let mut pasos = 0;
    loop {
        if numerito % 2 == 0 {
            numerito = numerito / 2;
        } else {
            numerito = numerito * 3 + 1;
        }
        pasos += 1;
        if numerito == 1 {
            break;
        }
    }
    println!("fueron necesarios dar {} pasos", pasos)
}
