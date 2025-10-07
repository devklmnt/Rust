fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    let values = [8, 30, 1, 3]; 
    let mut sum = 0;  

    // Acá la variable del loop n apunta a cada elemento del slice a medida que se itera.
    // En rust los arrays no pueden crecer, hay otras estructuras de datos con tamaño dinámico
    for n in &values[0..2] {
        // n es ahora una referencia (apuntador) a un elemento del array
        // *n dereferencia el apuntador para acceder al valor 
        sum = add(sum, *n); 
    }
    for n in &values[2..4] {
        sum = add(sum, *n);
    }

    println!("sum = {}",sum);
}
