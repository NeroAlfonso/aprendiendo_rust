# Juego de las adivinanzas
En este ejemplo se tendrán en cuenta conceptos varios sobre Rust.
> c_guessing_game/Cargo.toml
`
[package]
name = "c_guessing_game"
version = "0.1.0"
authors = ["nerioalfonso"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.3"
`
Se puede apreciar la única dependencia del proyecto, que será usada a continuación
> c_guessing_game/src/main.rs
`
use std::io; //libreria input output
use std::cmp::Ordering; //enum para comparaciones
use rand::Rng; //generador de numeros aleatorios
fn main()
{
    println!("Guess the number!");
    let secret_number =rand::thread_rng().gen_range(1..101); //genera un numero aleatorio por defecto u32 (numero sin signo de 32 bits), de 1 a 100 (el numero limite no se considera)
    println!("The secret number is: {}", secret_number);
    loop //inicia un bucle infinito a priori
    {
        println!("Please input your guess.");
        let mut guess =String::new(); //nueva variable mutable de tipo String vacia
        io::stdin() //recibiendo el input estandar de la terminal
            .read_line(&mut guess) //al leer la linea de terminal, 
            //asociando el valor a una referencia (&) mutable (mut) sobre la variable guess
            .expect("Failed to read line"); //si el result (enum Result) es Err, rompe el flujo del
            //programa y muestra el mensaje provisto. Si el Result retorna ok (junto con el tipo 
            //generico de la respuesta) se asigna a la referencia (a la variable guess)
        //let guess: u32 =guess.trim().parse().expect("Please type a number");
        let guess: u32 = match //match es una suerte de switch que evalua el valor y lo maneja con 
        //"brazos" (logica) en función de las distintas respuestas
        guess.trim().parse() //en este caso evaluará la respuesta de parse (un tipo enum Result)
        {
            Ok(num) => num, //retorna el valor convertido en caso de ser ok
            Err(_) => continue //o se salta una iteración del bucle
        };
        println!("Your guessed: {}", guess);
        match guess.cmp(&secret_number) //se evalua en valor que retorna la función cmp
        //que recibe una referencia (&) a la variable secret_number
        //el retorno es un enum tipo Ordering
        {
            Ordering::Less =>println!("Too small!"), //si es menor
            Ordering::Greater =>println!("Too big!"), //si es mayor
            Ordering::Equal => //si es el numero exacto
            {
                println!("You win!");
                break; //rompe el bucle, evitando la ejecución de una nueva partida 
                //(volver a pedir un numero para tratar de predecir el valor secreto)
            }
        }
    }
}
`  
A partir de este ejemplo podemos ver paso a paso.
## Use  
Por defecto rust considera algunos tipos basicos en el "preludio" (prelude), si requieres usar un tipo que no está presente en el "preludio" deberás importarlo al contexto o alcance usando la declaración "use".  
`
use std:io; //entradas y salidas estandar
use std::cmp::Ordering; //enum para comparaciones
use rand::Rng; //generador de numeros aleatorios
`
## Variables  
Para crear una variable se usa la declaración "let", estas incluyen caracteristicas como...  
- Son inmutables por defecto  
`
let apples =5; //su valor no se puede modificar
`  
- Pueden hacerse mutables con la declaración "mut"  
`
let mut apples =5; //su valor se puede modificar
`  
- A continuación, una variable con un valor vacío de tipo String mutable  
`
let mut guess =String::new();
`
- Las variables tienen inferencia de tipos (pueden determinar su tipo en función de los valores provistos)
`
    let secret_number =10; //por defecto es un u32 (valores de 32 bits sin signo)
`
- Las variables pueden ser sombreadas (shadowing), que es lo mismo que ser redefinidas en función de la utilidad que podamos ver en ellas como evitar variables auxiliares. Ahondado mas adelante
`
let guess: &str;
let guess: u32;
`
## Rand
Librería (library crate) que usaremos para generar números aleatorios. Se debe agregar a nuestras dependencias en *c_guess_game/Cargo.toml*
- Usaremos el rasgo (trait) Rng para generar números aleatorios, los rasgos se ahondarán mas adelante
`
let secret_number =rand::thread_rng().gen_range(1..101);
`  
*Este método genera un número en un rango inclusivo..exclusivo dado*

## Loop
Corresponde a un bucle infinito, a menos que lo detengas con "break". Para saltar a la siguiente iteración puedes usar "continue". Mas adelante, mas sobre loop.
`
    loop
    {
        ...
        if ...
        {
            continue;
        }
        ...
        ...
        if ...
        {
            break;
        }
    }
`
## Std
Librería que usaremos para manejar la entrada estandar desde la terminal.  
`
io::stdin().read_line(&mut guess).expect("Failed to read line");
`  
- Con esta línea podemos obtener lo que el cliente haya escrito en la terminal
- Con .read_line(&mut guess) podemos leer lo que se ha escrito en al terminal y asignarlo a una variable, que será la del parámetro o mas bien a la referencia (&) de una variables. Este método también retorna una enumeración (enums) de tipo "Result", que nos permite evaluar dos posibles valores "Ok" y "Err", donde "Ok" es acompañada del valor recibido.
- La enumeración "Result" tiene un método llamado "expect" que cierra el programa y muestra un mensaje provisto como parámetro, manejando el error producido, hasta cierto punto.
- Las referencias al igual que las variables son inmutables, de desear el comportamiento opuesto se debe proveer la palabra reservada "mut"

## Parse
`
guess.trim().parse()
`  
En esta línea estamos eliminando los espacios en blanco y tratando de convertirlo a númerico, el retorno del método parse es una enumeración Result, por lo que podemos usar su método expect para manejar el error o usar un match para evaluarlo mas detenidamente.

## Enum Ordering
Enumeración que usaremos para determinar si el resultado de una comparación es igual, sobrepasa o es menor al otro valor.
`
match guess.cmp(&secret_number) //se evalua en valor que retorna la función cmp
        //que recibe una referencia (&) a la variable secret_number
        //el retorno es un enum tipo Ordering
        {
            Ordering::Less =>println!("Too small!"), //si es menor
            Ordering::Greater =>println!("Too big!"), //si es mayor
            Ordering::Equal => //si es el numero exacto
            {
                println!("You win!");
                break; //rompe el bucle, evitando la ejecución de una nueva partida 
                //(volver a pedir un numero para tratar de predecir el valor secreto)
            }
        }
`  
- Con el método "cmp" podemos comparar una dos valores (de tipos compatibles), el resultado será una enumeración (enum) de tipo Ordering
- La ennumeración la manejaremos con un bloque match, donde en función de cada opción adoptaremos uno u otro comportamiento