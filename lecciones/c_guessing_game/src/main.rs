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