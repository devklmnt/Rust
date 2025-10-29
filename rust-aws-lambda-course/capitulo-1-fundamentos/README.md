# Capítulo 1: Fundamentos de Rust

## Objetivos del Capítulo

Al finalizar este capítulo serás capaz de:
- Entender la sintaxis básica de Rust 
- Dominar el sistema de ownership
- Usar borriwing y referencias correctamente 
- Trabajar con todos los tipos de datos básicos 
- Implementar control de flujo eficiente 

## ¿Por qué Rust?

Rust fue diseñado para ser **seguro**, **rápido** y **concurrente**. Para AWS Lambda, esto se traduce en:

### Ventajas para Serverless:
- **Cold Starts Ultra-Rápidos**: 50-150ms vs 1-3s de Java 
- **Uso Eficiente de Memoria**: Sin garbage collector
- **Cero Runtime Errors**: Catches errores en compiole time
- **Concurrencia Sin Fear**: No data races por diseño

---

## 1. Sintaxis Básica

### Instalación (si no tienes Rust)

```bash
# Instalar Rust 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verificar instalación 
rustc --version
cargo --version
```

### Tu Primer Programa 
```rust
fn main() {
    println!("¡Hola, futuro desarrollador de AWS Lambda!");
    
    // Comentarios de línea
    
    /* Comentarios 
        de bloque 
    */
}
```
### Variables y Mutabilidad 

```rust
fn main(){
    // Variables inmutables por defecto 
    let nombre = "CloudDeveloper";
    println!("Hola, {}", nombre);

    // Variables mutables - deben declararse explícitamente 
    let mut contador = 0;
    contador += 1;
    println!("Contador {}", contador);

    // Shadowing - redefinir variable con nuevo tipo
    let espacios = "   ";
    let espacios = espacios.len(); // Shora es un número 
    println!("Número de espacios: {}", espacios);
}
```

---

## 2. Tipos de Datos 

### Tipos Escalares 

```rust 
fn main(){
    // ============ ENTEROS ============
    let edad: u8 = 25;  // 0 a 255
    let poblacion: u64 = 1000000; // Números grandes 
    let temperatura: i32 = -10; // Números con signo 

    // ============ FLOTANTES ============
    let precio: f64 = 29.99; // Precisión doble (por defecto)
    let ratio: f32 = 3.14; // Precisión simple 

    // ============ BOOLEANOS ============
    let es_activo: bool = true;
    let tiene_permiso =  false; // Inferencia de tipo 

    // ============ CARACTERES ============
    let emoji: char ='a';
    let letra: char = 'R';

    println!("Edad: {}, Precio: {}, Activo: {}, Emoji: {}", edad, precio, es_activo, emoji);
}
```

### Tipos Compuestos 

```rust
fn main() {
    // ============ TUPLAS ============
    let servidor_info: (String, u16, bool) = (
        String::from("aws-lambda-rust"),
        443,
        true
    );

    // Destructuring
    let (nombre, puerto, ssl) = servidor_info;
    println!("Servidor: {} en el puerto {}, SSL: {}", nombre, puerto, ssl);

    // Acceso por índice
    let mi_tupla = (100, "datos", true);
    println!("Puerto: {}", mi_tupla.1);

    // ============ ARRAYS ============
    let regiones: [&str; 4] = ["us-east-1","us-west-2","eu-west-1","ap-southeast-1"];
    let numeros = [1, 2, 3, 4, 5];
    let ceros = [0; 10]; // Array de 10 ceros

    println!("Primera región: {}", regiones[0]);
    println!("Tamaño del array: {}", numeros.len());
}
```

## 3. Ownership - el Corazón de Rust

### Reglas de Ownership

1. **Cada valor tiene un owner**
2. **Solo puede haber un owner a la vez**
3. **Cuando el owner sale de scope, el valor se libera**

```rust
fn main() {
    // ============ OWNERSHIP BÁSICO ============
    {
        let mensaje = String::from("Hola AWS Lambda"); // mensaje entra en scope
        println!("{}", mensaje);
    } // mensaje sale del scope y se libera automáticamente 

    // ============ MOVE SEMANTICS ============
    let datos1 = String::from("Datos importantes");
    let datos2 = datos1; // datos1 se mueve a datos2

    // println!("{}", datos1); // ERROR: datos1 ya no es válido
    println!("{}", datos2); // OK: datos2 es el nuevo owner
    
    // ============ COPY PARA TIPOS SIMPLES ============
    let x = 5;
    let y = x; // Los enteros implementan Copy
    println!("x: {}, y: {}", x, y); // Ambos son válidos
}

// Función que toma ownership 
fn procesar_datos(datos: String) {
    println!("Procesando: {}", datos);
} // Datos se libera aquí

// Función que devuelve ownership
fn crear_mensaje() -> String {
    String::from("Mensaje desde Lambda");
}
```

### Ejemplo Práctico: Simulador de Request Lambda 

```rust
fn main() {
    let request_body = String::from(r#"{"user_id": 123, "action": "create"}"#);

    // Procesamos el request (se mueve la ownership)
    let response = procesar_request_lambda(request_body);
    println!("Response: {}", response);

    // request_body ya no es accesible aquí 
}

fn procesar_request_lambda(body: String) -> String {
    println!("Procesando request: {}", body);

    // Simulamos procesamiento
    if body.contains("create") {
        format!(r#"{{"status": "success", "message": "Recurso creado"}}"#)
    } else {
        format!(r#"{{"status": "error", "message": "Acción no válida"}}"#)
    }
}
```

## 4. Borrowing y Referencias 

### Referencias Inmutables 
```rust
fn main() {
    lambda_config = Sting::from("runtime=rust, memory=128MB");

    // Presentamos la referencia (no movemos ownership)
    let config_length = calcular_longitud(&lambda_config);

    println!("Config: {}", lambda_config); // Aún podemos usar lambda_config
    println!("Longitud: {}", config_length);
}

fn calcular_longitud(config: &String) -> usize {
    config.len()
}// config sale de scope, pero no libera memoria (es solo una referencia)

```

### Referencias Mutables 
```rust
fn main() {
    let mut response = String::from(r#"{"status": "pending"}"#);

    // Modificamos a través de referencia mutable
    actualizar_response(&mut response);

    println!("Response actualizado {}", response);
}

fn actualizar_response(response: &mut String) {
    response.clear();
    response.push_str(r#"{"status": "completed", "timestamp": "2025-10-22T10:00:00Z"}"#);
}
```

### Reglas de Borrowing 
```rust
fn main() {
    let mut datos = String::from("datos lambda");

    // Múltiples  referencias inmutables 
    let ref1 = &datos;
    let ref2 = &datos;
    println!("{} y {}", ref1, ref2);

    // Una referencia mutable (cuando las inmutables ya no se usan)
    let ref_mut = &mut datos;
    ref_mut.push_str(" - modificados");
    println!("{}", ref_mut);

    // NO se pueden mezclar referencias mutables e inmutables 
    // let ref2 = &datos; // Error si ref_mut sigue en uso
    // println!("{}", ref_mut); Error si ref3 existe
}
```

---

## 5. Control de Flujo

### If/Else Expressions 

```rust
fn main() {
    let http_status = 200;

    // if como expresión
    let mensaje = if http_status == 200 {
        "Éxito"
    } else if http_status == 404 {
        "no encontrado"
    } else {
        "Error en el servidor"
    };

    println!("Status {}: {}", http_status, mensaje);

    // Ejemplo lambda: validar región AWS 
    let region = "us-east-1";
    let es_region_valida = if region.starts_with("us-") ||
                              region.starts_with("eu-") ||
                              region.starts_with("ap-") {
                                true
                              } else {
                                false
                              };
    println!("Región {} es válida {}", region, es_region_valida);
}
```

### Loops 
```rust
fn main() {
    println!("=== LOOP INFINITO ===");
    let mut contador = 0;
    let resultado = loop {
        contador += 1;
        if contador == 3 {
            break contador * 10; // loop puede retornar valores
        }
    };
    println!("Resultado del loop: {}", resultado);

    println!("\n=== WHILE ===");
    let mut intentos = 5;
    while intentos > 0 {
        println!("Reintentando conexión AWS... intentos restantes: {}", intentos);
        intentos -= 1;
    }

    println!("\n=== FOR ===");
    let regiones = ["us-east-1", "us-west-2", "eu-west-1"];

    // Iterando sobre array
    for region in regiones.iter() {
        println!("Desplegando en región: {}", region);
    }

    // Rango númerico
    for i in 1..=5 {
        println!("Procesando batch {}", i);
    }

    // Con enumerate para obtener índice
    for (index, region) in regiones.iter().enumerate() {
        println!("Región {}: {}", index + 1, region);
    }
}
```

## 6. Funciones 

### Funciones Básicas 

```rust
fn saludar() {
    println!("¡Hola desde una función lambda!");
}

// Función con parámetros 
fn calcular_costo_lambda(invocaciones: u32, duracion_ms: u32) -> f64 {
    let costo_por_invocacion = 0.0000002; // $0.0000002 por invocación
    let costo_por_ms = 0.000000208; //$0.000000208 por 100ms

    let costo_invocaciones = invocaciones as f64 * costo_por_invocacion;
    let costo_duracion = (duracion_ms as f64 / 100.0) * costo_por_ms;

    costo_invocaciones + costo_duracion
}

// Función que retorna múltiples valores 
fn analizar_response(status: u16) -> (bool, String) {
    let es_exitoso = status >= 200 && status < 300;
    let mensaje = match status {
        200 => String::from("OK"),
        201 => String::from("Created"),
        400 => String::from("Bad Request"),
        404 => String::from("Not Found"),
        500 => String::from("Internal Server Error"),
        _ => String::from("Unknown Status"),
    };
    (es_exitoso, mensaje)
}

fn main() {
    saludar();

    let costo = calcular_costo_lambda(1000,500);
    println!("Costo estimado: ${:.8}", costo);

    let (exitoso, mensaje) = analizar_response(201);
    println!("¿Exitoso?: {},  Mensaje: {}", exitoso, mensaje);
}
```