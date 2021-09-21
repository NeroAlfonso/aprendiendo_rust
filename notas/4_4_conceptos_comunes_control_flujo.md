# Control de flujo
Ejecutar un código u otro, repetir un bloque lógico.
> lecciones/f_control_flow/src/main.rs
`
fn main() {
    //condicional if
    let number =9;
    if number ==10 
    {
        println!("igual a diez");
    }
    else if number > 10
    {
        println!("mayor a diez");
    }
    else
    {
        println!("menor a diez");
    }
    let foo = if false { 10 } else { 20 }
    println!("valor final: {}", foo);
    //bucles
    let mut iterations :u8 =10;
    loop
    {
        if iterations ==0 { break; }
        println!("iteración {}", iterations);
        iterations =iterations-1;
    }
    let mut counter :u8 =0;
    let ten = loop
    {
        counter +=1;
        if counter ==5
        {
            break counter *2;
        }
    };
    println!("valor de ten {}", ten);
    let mut counter =0;
    while counter <10
    {
        counter +=1;
        println!("contador actual {}", counter);
    }
    //bucle for
    let number_array :[u8; 10] =[1, 2, 3, 4, 5, 6, 7, 8, 8, 9];
    for element in number_array.iter() //.iter() iteración
    {
        println!("iteración: {}", element);
    }
    for element in (1..4).rev() //.rev() iteración en reversa 
    //(1..4) es un rango que va desde el 1 hasta el 3 (no se considera el final del rango)
    {
        println!("cuenta regresiva: {}", element);
    }
}
`  
## Condicional if  
- Evalua una condición, si se cumple se ejecuta el bloque siguiente a "if" sino "else". Si agregas "else if" al bloque siguiente del "if", puedes evaluar mas de una condición (opcional).
`
let number =9;
    if number ==10 
    {
        println!("igual a diez");
    }
    else if number > 10
    {
        println!("mayor a diez");
    }
    else
    {
        println!("menor a diez");
    }
`  
- Los retornos distintos a booleanos no son admitidos por el condicional "if" (solo condiciones y booleanos). El código a continuación sería un código invalido.  
`
    let number = 3;
    if number { //evaluación incorrecta
        println!("number was three");
    }
`  
- Puedes usar el condicional para definir una variable, todos los posibles retornos del condicional deben corresponder a un mismo tipo, en conclusión la variable puede ser de un solo tipo, por inferencia o definición explícita de la variable.  
`
 let number = if condition { 5 } else { 6 };
`  
## Bucles  
### Loop  
- Son infinitos hasta que un "break" rompe con la iteración infinita.  
`
let mut iterations :u8 =10;
    loop
    {
        if iterations ==0 { break; }
        println!("iteración {}", iterations);
        iterations =iterations-1;
    } 
`  
- Pueden tener retornos.  
`
let mut counter :u8 =0;
    let ten = loop
    {
        counter +=1;
        if counter ==5
        {
            break counter *2;
        }
    };
`  
### While  
- Se repetirá hasta que su condición no aplique.  
`
let mut counter =0;
    while counter <10
    {
        counter +=1;
        println!("contador actual {}", counter);
    }
`  
### For  
- Recorre iterables (.iter()) y usa una variable auxiliar que adopta el valor de la iteración actual.  
`
let number_array :[u8; 10] =[1, 2, 3, 4, 5, 6, 7, 8, 8, 9];
    for element in number_array.iter() //.iter() iteración
    {
        println!("iteración: {}", element);
    }
`  
- Podemos recorrer al revés invirtiendo la iteración con ".rev()".  
`
    for element in (1..4).rev() //.rev() iteración en reversa 
    //(1..4) es un rango que va desde el 1 hasta el 3 (no se considera el final del rango)
    {
        println!("cuenta regresiva: {}", element);
    }
`  
> actividades por hacer: Convert temperatures between Fahrenheit and Celsius, Generate the nth Fibonacci number, Print the lyrics to the Christmas carol “The Twelve Days of Christmas,"taking advantage of the repetition in the song.  
*Nota*  