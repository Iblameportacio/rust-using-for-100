//pirámide centrada de asteriscos de n filas
fn main() {
    let filas_totales = 20;

    for i in 1..=filas_totales {
        // 1. Calculamos cuántos espacios y asteriscos tocan en esta fila
        let espacios_necesarios = filas_totales - i;
        let asteriscos_necesarios = (2 * i) - 1;

        // 2. Creamos los hilos de texto repitiendo los caracteres
        let bloques_espacio = " ".repeat(espacios_necesarios);
        let bloques_asterisco = "*".repeat(asteriscos_necesarios);

        // 3. Juntamos ambos e imprimimos
        println!("{}{}", bloques_espacio, bloques_asterisco);
    }
}
