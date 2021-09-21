# Funciones
> lecciones/e_functions/src/main.rs
`
fn main() {
    println!("Hello, world!");
    another_function();
}
fn another_function()
{
    println!("Otra función");
    another_function_with_parameters(5);
}
fn another_function_with_parameters(x :u32)
{
    println!("Parámetro {}", x);
    statements_and_expressions();
}
fn statements_and_expressions()
{
    let var =10;
    //let var =(let foo =10);
    let var ={
        let foo =24;
        foo //las expresiones no llevan punto y coma al final
    };
    println!("var: {}", var);
    print!("función con retorno {}", function_with_return());
}
fn function_with_return() -> u8 {5}
`  
- Se definen con la palabra reservada "fn" seguida del nombre.
`
fn main() {
    println!("Hello, world!");
}
`  
- Pueden recibir parámetros, estos se deben especificar entre paréntesis con nombre y tipo.  
`
fn another_function_with_parameters(x :u32)
{
    println!("Parámetro {}", x);
}
`  
- Cuando una función retorna un valor se especifica el tipo de retorno posterior al nombre de la función *fn nombre () -> tipo_retorno*.  
`
fn function_with_return() -> u8 {5}
`  
Las funciones se componen de una serie de declaraciones y opcionalmente terminan en una expresión.  
## Declaraciones (statements)  
- Las declaraciones no retornan valores.  
- Declarar una variables es una declaracion.  
`
    let success :bool =true;
`  
- Por tanto el siguiente código es invalido.  
`
    let aux_success : bool = (let success: bool =true);
`  
- Las declaraciones llevan ";" al final.  
## Expresiones (expressions)  
- Las expresiones retornan valores.  
- Los bloques de ambito se consideran expresiones.  
`
let var ={
        let foo =24;
        foo //retorno
    };
`  