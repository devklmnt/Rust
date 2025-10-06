
// Se crea una función para sumar
// recibe como parámetros a y b del tipo i32 y retorna un i32
fn add(n1: i32, n2: i32) -> i32 {
    //return n1 + n2;
    // el valor del return es el ultimo valor ejecutado antes que la función finalice
    // por esto se puede omitir el return y unicamente dejar la expresion a evaluar sin ;
    n1 + n2
}

fn main() {
    let a: i32 = 38; 
    let b: i32 = 4;
    let mut sum: i32 = add(a,b);  // Se llama a la función add

    println!("{} + {} = {}", a, b, sum);
    // Salida: 38 + 4 = 42

    sum = add(sum,1); 
    println!("{}",sum);
}
