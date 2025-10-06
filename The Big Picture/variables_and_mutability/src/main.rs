fn main() {
    // las variables a, b son inmutables por defecto
    let a: i32 = 38; //i32 se puede omitir ya que rust posee inferencia de tipos
    let b: i32 = 4;
    let mut sum: i32 = a + b; // la variable sum es mutable 

    println!("{} + {} = {}", a, b, sum);
    // Salida: 38 + 4 = 42

    sum +=b; // Al agregar mut a la declaracion, se permite modificar el valor de la variable
    println!("{}",sum);
}
