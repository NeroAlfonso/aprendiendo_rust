fn main() {
    println!("Hello, world!");
    another_function();
}
fn another_function()
{//la funciones se declaran con la palabra reservada "fn" seguido de su nombre
    println!("Otra función");
    another_function_with_parameters(5);
}
fn another_function_with_parameters(x :u32)
{
    println!("Parámetro {}", x);
    statements_and_expressions();
}
//===declaraciones y expresiones===
fn statements_and_expressions()
{
    //declarar una variable es una declaración
    let var =10;
    //las declaraciones no tienen retorno por lo tanto el siguiente código es invalido
    //let var =(let foo =10);
    //los bloques de ambito ({}) tambien se consideran expresiones
    let var ={
        let foo =24;
        foo //las expresiones no llevan punto y coma al final
    };
    println!("var: {}", var);
    print!("función con retorno {}", function_with_return());
}
//cuando una función retorna un valor se especifica el tipo de retorno posterior al nombre de la función
fn function_with_return() -> u8 {5}
