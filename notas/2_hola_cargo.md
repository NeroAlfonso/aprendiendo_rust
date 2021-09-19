# Hola cargo
Cargo es el sistema de construcción y manejo de dependecias de rust, construye tu código, descarga dependencias y las construye (Las dependencias son esas librerias que tu código necesita).  
`
> cargo --version
> cargo new b_hello_cargo
`  
Esto último creará una carpeta, con el nombre del proyecto y dentro dos archivos...
> b_hello_cargo/Cargo.toml
`
[package]
name = "b_hello_cargo"
version = "0.1.0"
authors = ["nerioalfonso"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
`  
El anterior es el formato de archivo de configuración de rust (.toml "Tom’s Obvious, Minimal Language")   
- La sección "paquete" (package) indica que a continuación se configura uno. Cargo necesita para compilar tu código el nombre (name), la versión (version) y la edición de rust (edition).
- La sección "dependencias" (dependencies) describiría justo debajo las dependencias del proyecto.
- En rust, los paquetes son llamados "crates".
> b_hello_cargo/src/main.rs
`
fn main() { //función punto de entrada
    println!("Hello, world!"); //macro
}
`  
- Cargo genera un "Hello, world!" para tí por defecto.  
Cargo considera que tu código siempre vivirá dentro del directorio *src*, el directorio raíz es para archivos de configuración, licencia, README y similes.  
## Compilar el proyecto con cargo.
`
> cargo build
`  
Esto generará con ejecutable en el directorio *target/debug/b_hello_cargo* o *...rgo.exe* en windows.
También se ha generado un archivo *Cargo.lock* con las dependencias exactas usadas en el proyecto, normalmente este archivo nunca deberá de modificarlo manualmente.  
## Compilar y ejecutar.
`
> cargo run
`  
## Verificar tu código sin generar un ejecutable
`
> cargo check
`  
*proporciona una verificación rapida del código*
## Compilar para producción
`
> cargo build --release
`  
Para cuando termine la ejecución de la instrucción se habrá generado un archivo *target/release/b_hello_cargo* o *...rgo.exe* en Windows.
`
> cargo doc --open
`  
Te muestra la documentación del proyecto en cuestión.