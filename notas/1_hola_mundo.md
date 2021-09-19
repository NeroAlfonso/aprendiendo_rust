# Hola mundo  
A la hora de crear un archivo .rs para nuestro código, la convención en rust es usar snake case (palabra_palabra.rs).
Para compilar los archivos .rs usamos el comando rustc (rust compiler) luego lo ejecutamos nativamente.  
`
> rustc main.rs
> ./main
`  
> a_hello_world/main.rs  
`
fn main() { //función punto de entrada
    println!("Hello, world!"); //macro
}
`  
- Una función en rust se define con la palabra reservada "fn", seguido del nombre de la función.
- En el ejemplo se puede ver la función main, que es la función principal de tus programas, no recibe parametro y no cuenta con ningun retorno, de considerar parámetros estos irían dentro de los parentesis.
- En el estilo de rust la forma de sangrar es con 4 espacios, no una tabulación.
- println! no es una función, es una macreo y tiene sus diferencias con las funciones (mas a fondo a futuro).
- Las expresiones en rust (y otros lenguajes) terminan con un punto y coma (;).
- rustc genera un archivo ejecutable (Linux y macOS). En Windows ademas de este, genera un archivo con información de depuración (.pdb)