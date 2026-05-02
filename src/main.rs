//Verifica si un número es "fuerte"
//  (la suma de los factoriales de sus dígitos es igual al número).
fn main() {
    let mut suma_total = 0; //to save the sum of the factorials
    let mut num = 145; //this is the number to analisys
    let original_num = num; //just for compare the num later without problems
    println!(" el resultado de {} es:", num); //just printing the num to analisys
    while num > 0 {
        //while the number num > 0
        let mut factorial = 1; //because each digit will have its own factorial
        let ultimo_digito = num % 10; //// get the last digit of num
        for i in 1..=ultimo_digito {
            // calculate the factorial of the digit
            factorial *= i; //
        }
        num = num / 10; //remove the final digit that i used in the past line of code
        suma_total += factorial; // add the factorial to the total sum
    } // when no digits are left (num = 0), exit the loop
    println!("{} ", suma_total); //simply printing the total sum of the factorials
    if original_num == suma_total {
        println!("es fuerte")
    } else {
        println!("no es fuerte")
    }
}
//first time comenting rust code
