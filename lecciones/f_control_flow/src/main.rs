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
    //if en declaraciones
    let foo = if false { 10 } else { 20 }; //los retornos deben obedecer al tipo de la variable
    //y en caso de inferencia de tipos, deben ser retornos del mismo tipo en todas las condiciones
    println!("valor final: {}", foo);
    //bucles
    let mut iterations :u8 =10;
    loop
    {
        if iterations ==0 { break; }
        println!("iteración {}", iterations);
        iterations =iterations-1;
    }
    //los bucles pueden tener retornos
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
