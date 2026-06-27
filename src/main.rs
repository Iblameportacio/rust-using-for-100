//Suma de dígitos de un número (ej: 1234 → 10)
fn main() {
    let mut numero = 1234;
    let mut suma = 0;

    // El bucle se repite mientras el número conserve dígitos
    while numero > 0 {
        let ultimo_digito = numero % 10;

        // 2. Súmalo a tu variable acumuladora 'suma'
        suma += ultimo_digito;

        // 3. Quítale el último dígito al número dividiéndolo entre 10
        numero /= 10;
    }

    println!("La suma de los dígitos es: {}", suma); // Debería dar 10
}
