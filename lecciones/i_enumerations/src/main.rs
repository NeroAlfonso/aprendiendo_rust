#[derive(Debug)] //agregando esta anotación jussto antes de la definición de la estructura (struct)
//le damos el rasgo (trait) Debug a la misma, para que pueda formatearse en caso de querer imprimirla
enum UsState
{
    Alabama,
    Alaska
}
enum Coin
{
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}
enum Message
{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}
impl Message //la forma de implementar metodos a las enumeraciones (enum) es igual que con las
//estructuras (structs)
{
    fn call(&self) //incluida la referencia hacia la instancia misma como primer parámetro
    {
        println!("Llamando!");
    }
}
enum IpAddrKind //una enumeración inicio por la palabra reservada enum, seguido del nombre
{//en el cuerpo, se definen lo posibles estados
    V4,
    V6
}
enum IpAddrWithValue
{
    V4(String), //un valor de enum puede tener asociado un valor adjunto
    //este puede ser un tipo escalar, o complejo, includas estructuras
    V6(String)
}
enum IpAddrWithMulValue
{
    V4(u8, u8, u8, u8), //la diferencia de tipos entre las opciones incluso cuando
    //comparten valor es algo que no se puede hacer con una estructura (struct)
    V6(String)
}
struct IpAddr
{
    kind :IpAddrKind, //la enumeración (enum) como tipo de un atributo en la estructura
    address :String

}
fn main()
{
    println!("Enumeraciones");
    let six =IpAddrKind::V6; //se pueden crear instancias de los valores de la enumeración
    //y todos los valores son del mismo tipo (IpAddrKind), para por ejemplo usarlo en una
    //función sin importar el valor sino el tipo
    let four =IpAddrKind::V4;
    route(&six);
    route(&four);
    //las enumeraciones al ser tipos pueden darle ssu naturaleza a cualquier atributo de una
    //estructura
    let home =IpAddr
    {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback =IpAddr
    {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    let homeValue =IpAddrWithValue::V4(String::from("127.0.0.1")); //los valores se pasan al valor
    //de la enumeracíon como si se tratara de un constructor
    let loopbackValue =IpAddrWithValue::V6(String::from("::1"));
    let homeMulValue =IpAddrWithMulValue::V4(127, 0, 0, 1); //el valor V4 de la enumeración recibe
    //distintos datos que en el caso de V6
    let loopbackMulValue =IpAddrWithMulValue::V6(String::from("::1")); //donde solo recibe un String
    let msg :Message =Message::Write(String::from("lelelaloli"));
    msg.call();
    //==Ausencia de nulos
    //Rust no tiene valores nulos, para evitar que se presenten errores sin concepto alguno
    //(un valor nulo, puede deberse a muchos errores, que no fué provisto el valor, que algún proceso falló, entre otros
    //, el valor nulo no representa un concepto de error, sino que es una propagación generica de multiples errores)
    //en su lugar usa una implementación de una enumeración (enum)
    //llamada Option para representar en concreto un valor usente o existente
    //NOTA: El problema es realmente originado por la implementación de null (manejo de nulos)
    //https://doc.rust-lang.org/std/option/enum.Option.html
    //Al incluirse Option en el preludio, no es necesario importarlo ni usar el prefijo Option::
    let some_number : Option<u8> =Some(1);
    let other_some_number =Some("a string"); //inferencia de tipo
    //let other_other_some_number =None; //no puede inferir el tipo, porque no hay un dato provisto
    let other_other_some_number : Option<String> =None;
    //en lo practico, la diferencia entre usar nulo y Option es que el compilador, no permitirá
    //usar la variable some_number como el valor final contenido en Option::Some(T), porque
    //son tipos diferentes
    let number : u8 =5;
    //let some_number : u8 = some_number + number; //esto provocaria un error
    //por lo tanto, el compilador nunca asumirá que un valor no es nulo cuando en realidad
    //lo es, que es uno de los mayores problemas de nulo. En su lugar te obliga a verificar
    //con el tipo Option si el valor existe o no, antes de usarlo, permitiendo así que el
    //valor, siempre sea valido. Option cuenta con una serie de métodos para trabajar con el
    //disponibles en la documentación
    //==control de flujo, condicional match
    let coin : Coin =Coin::Quarter(UsState::Alabama);
    println!("centimos {}", value_in_cents(&coin));
    //==coincidiendo con Option
    let one =Some(1);
    let two =plus_one(one);
    let none =plus_one(None);
    //==manejador generico para los casos no manejados
    let x :u8 =10u8;
    match x
    {
        0 => println!("zeero!"),
        10 =>println!("ten!"),
        //_ =>() //para todas las opciones no manejadas se ejecutará este código
        //() indica que no pasará nada en estos casos
        _ => println!("Otro!") //con un comportamiento asociado
    }
    //==control conciso con if let
    let some_value =Some(4u8);
    if let Some(3) =some_value 
    {
        println!("if Three");
    }
    else
    {
        println!("its other");
    }
    if let Coin::Quarter(state) =coin //accediendo al valor del estado de la enumeración (enum)
    {
        println!("Quarter {:?}", state);
    }
}
fn plus_one(number : Option<u8>) -> Option<u8>
{
    match number
    {
        None => None,
        Some(auxNumber) => Some(auxNumber +1)
    }
}
fn value_in_cents(coin :&Coin) -> u8
{
    match coin //recibe un valor a comparar
    {
        Coin::Penny => //si coincide con la condición
        //todos los posibles estados de la enumeración (enum), deben ser manejados
        {//ejecuta el código del brazo (arm)
            println!("Penny!"); 
            1
        },
        Coin::Nickel => 5, //igual con el resto...
        Coin::Dime => 10,
        Coin::Quarter(usState) => //para acceder a los valores asociados de la enumeración
        //se recibe como un parámetro
        {
            println!("state {:?}", usState);
            25
        }
    }
}
fn route(ipAddrKind: &IpAddrKind)
{
    match ipAddrKind
    {
        IpAddrKind::V4 => println!("ipv4"),
        IpAddrKind::V6 => println!("ipv6")
    }
}