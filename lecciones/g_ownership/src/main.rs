//===propiedad (ownership)===
    //==pila
    //almacena todo lo conocido en tiempo de compilación, con tamaño fijo
    //está estructurada en orden lifo
    //de acceso rápido
    //poner datos en la pila se denomina pushing y remover popping
    //==monton
    //menos veloz
    //almacena datos que estarán disponibles en tiempo de ejecución, o son de valor variable
    //ingresar valores se denomina allocating (asignar
fn main() {
    second_function();
    function_move();
    function_clone();
    ownership_and_functions();
    return_values_and_scope();
    references_and_borrowing();
    slices();
}
//cada valor tiene una variable llamada propietario (owner),
//solo puede existir un propietario a la vez
//Cuando el propietario se sale del ambito el valor es eliminado
fn second_function()
{//ambito
    //foo no ha sido declarado, por lo tanto no está disponible
    let mut foo :String =String::from("lelelaloli"); //el tipo string es util para casos en los que
    //no se conoce el valor final del dato alfanumerico o este es variable. Se asigna al montón
    //foo está disponible
    foo.push_str(" juesjues"); //agrega substring al final del String
    println!("{}", foo);
}//final del ambito 
//foo está fuera del ambito, por lo tanto ha sido eliminado, y ssu memoria ha ssido liberada (drop).  Resource Acquisition Is Initialization (RAII)
//===interacción entre variables y datos===
    //==moverse (move)
    fn function_move()
    {
        println!("función move");
        //como esto valores son simples enteros de valor conocido se guardan en la pila (stack)
        //y ambas variables tienen el mismo valor en distintas locaciones (deep copy)
        //ya que por su disponibilidad en la pila (stack) la resolución de la operación es muy rápida
        //por lo que se omite la llamada al metodo .clone porque no hay diferencia apreciable entre 
        //copia profunda o superficial en este caso (deep copy and shallow copy)
        //a esto se le conoce como rasgo de copia, y lo implementan todos loss tipos escalares y 
        //las tuplas (que usen tipos con el rasgo de copia)
        let bar =5;
        let foo =bar;
        let bar =bar+5;
        println!("foo {} bar {}", foo, bar); //ambos disponibles
        //en el caso del String que se aloja en el montón, no se hace un copia del valor de la variable
        //sino que el apuntador de la variable se transmite a la segunda y se desacopla de la anterior
        //evitando así que se liberen ambas variables que apunten a la misma locación en memoria
        //double free error
        let x =String::from("Hello");
        let y =x;//a partir de aquí, Rust ya no considera x como valida
        //similar a una copia superficial (shallow copy) pero Rut invalida la variable anterior
        //llamandole moverse (move)
        //println!("x {} y {}", x, y); //esta intrucción provocará un error
        println!("y {}", y);
        //NOTA: Rust nunca crea copias profundas (deep copy) para el montón automaticamente
    }//y esta fuera del ambito, por lo tanto es liberada
    //==clonar (clone)
    fn function_clone()
    {
        println!("función clonar");
        //para hacer copias profundas (deep copy) se usa el metodo clone
        //estas copias usan nuevas localidades de memoria para la segunda variable, 
        //compartiendo el valor, pero de forma independiente
        let x : String =String::from("hola");
        let y : String =x.clone(); //por la naturaleza de la copia profunda, es mas costoso
        //este proceso, cantidad de memoria y saltos en la busqueda de la memoria contribuyen a este
        //problema
        //a la hora de leer el código es un indicador claro de que algo distinto, que requiere de 
        //este comportamiento está sucediendo
        println!("x {} y {}", x, y);
    }
//===Propiedad y funciones===
fn ownership_and_functions()
{
    println!("propiedad y funciones");
    let s :String =String::from("hola"); //disponible para el ambito
    takes_ownership(s); //al pasar una variable a una función, esta actua de la misma forma
    //que la asignación de una variable a otra, en este caso aplica movere (move). Copia 
    //superficial (shallow copy) e invalidación de la variable anterior
    //al sser un tipo String, todo esto ocurre en el montón
    //println!("{}", s); //por lo tanto esta declaración provocará un error
    let i =10;
    makes_copy(i); //al entero ser un tipo escalar, tiene rasgo de copia, por lo tanto
    //hace una copia profunda, dentro de la pila (stack)
    println!("{}", i);
}
//===Retornando valores y ambito===
fn return_values_and_scope()
{
    println!("Retornando valores y ambito");
    let x =gives_ownership(); //esta funcióon espera retornar un String, pasando la propiedad
    //de dicho valor a la variable que espera el retorno de la función
    println!("x {}", x);
    let y =takes_and_gives_back(x);
    println!("y {}", y);
    //si quisieras tomar la propiedad de la variable parámetro y ademas usar el valor de retorno
    //de la función, puedes retornar una tupla
    let s :String =String::from("hello");
    let (s, len) =calculate_length(s); //esta variable retorna una tupla, que es manejada por la
    //destructuración para asignarse a las variables necesarias
    println!("s {} len {}", s, len);
}
//===referencias y prestamo===
fn references_and_borrowing()
{
    println!("Referencias y préstamo");
    //las referencias funcionan para no ceder la propiedad de la variable parámetro
    //al ambito de la función
    let s :String =String::from("hello");
    let len =calculate_length_ref(&s); //la referencia indica que el parámetro de la función
    //apuntará a la variable suministrada sin tomar propiedad de ella
    //a toda esta dinamica se le conoce como prestamo (borrowing)
    println!("s {} len {}", s, len);
    let mut s2 :String =String::from("hello"); //creamos una variable mutable
    let len =calculate_length_ref_mutable(&mut s2); //le pasamos a la función una referencia mutable
    //por lo tanto podrá cambiar el parámetro provisto
    println!("s {} len {}", s2, len); //aquí se puede evidenciar
    //solo puede existir una referencia mutable hacia una misma variable en un mismo ámbito
    let mut s3 :String = String::from("lelele");
    let s4 =&mut s3;
    let s5 =&mut s3;
    //println!("s4 mut {} s5 mut {}", s4, s5); //esta declaración provocaría un error
    //esta restricción previene las carreras de dato (data races) que son comportamientos
    //en la lectura/escritura paralela que pueden ocasionar problemas
    //para usar mas de una referencia mutable hacia la misma variable se puede crear un nuevo ámbito
    {
        let s4 =&mut s3;
        println!("nuevo ámbito s4 {}", s4);
    }
    let s5 =&mut s3;
    println!("nuevo ámbito s5 {}", s5);
    //la combinación de referencias mutables e inmutables en un mismo ámbito también es una
    //restricción
    let mut s6 :String = String::from("lelele");
    let s7 =&s6;
    let s8 =&mut s6; //habilitar esta linea generaría un error
    //println!("{} {}", s7, s8); //aquí dispararía el error
    let s8 =&s6; //sin embargo dos referencias inmutables si están admitidas, debido a que no se
    //espera que ninguna cambie mientras se está leyendo
    //el alcance de las referencias van desde la línea donde se definieron hasta la ultima vez que
    //se usaron, por ejemplo, sería un caso valida el siguiente código
    let s9 =&s6;
    println!("{}", s9);
    let s10 =&mut s6;
    println!("{}", s10);
    //===referencia colgante (dangling references)
    //significa que una referencia apunta a un valor que ya no existe
    //let ref_str : &String =get_reference_string(); //esta linea muestra el error
    let aux_str : String =get_string_with_ownership(); //esta función está retornando el String
    //y no una referencia a una localización de memoria ya liberada (drop), suministrando así
    //también la propiedad a la variable
}
//===rebanadas===
fn slices()
{
    println!("rebanadas");
    let mut s :String =String::from("hello world");
    let index =first_word(&s); //retorna el indice de la primera palabra que encuentra (espacio)
    s.clear(); //limpia el String, dejandolo equiparable a ""
    //pero index, sigue teniendo el indice de la primera palabra, aun cuando la palabra en si misma
    //ha sido complemetamente limpiada y ya no es un indice valido en el String
    println!("index firstvword {}", index);
    s =String::from("hello world");
    //las rebanadas o sectores vienen a resolver este problema (slices)
    let hello =&s[0..5]; //se especifica el indice de inicio y el de final en un rango.
    //internamente el sector (slice) calcula el tamaño del rango operando indice final menoss indice inicial
    
    let hello =&s[..5];//esta sintaxis también es valida para omitir el cero, y puede usarse cuando
    //queremos iniciar el rango en la primera posición del string
    let world =&s[6..11];
    let world =&s[6..s.len()]; //si quisieramos obtener el ultimo indice de forma dinámica
    let world =&s[6..]; //esta sintaxis es valida también para apuntar al último indice del String
    let entire_string =&s[0..s.len()]; //para tomar el String entero
    let entire_string =&s[..]; //si quisieramos tomar un rango del inicio al fin del String
    //también es valida esta sintaxis
    let word =first_word_slice(&s);
    //s.clear(); //esta instrucción provocará un error, debido a que .clear() requiere una referencia
    //mutable para limpiar el String, sin embargo, nuestro sector (slice) word es una referencia 
    //inmutable que está en uso despues de la instrucción .clear() por lo tanto continua siendo valida
    //así podemos validar nuestros sectores con una vinculación directa al String en cuestión, provocando
    //errores en tiempo de compilación y anticipando posibles errores
    println!("primera palabra {}", word);
    let x ="Hello, World"; //los String literales que se almacenan en el binario, en realidad
    //son sectores que apuntan a una sección del mismo, y son inmutables porque el sector es inmutable
    //&str
    //podemos generalizar la función para que pueda recibir mas organizamente Strings y strings literales
    //convirtiendo el parámetro en un tipo sector de string (&str -> slice string)
    let string_word =first_word_slice_general(&s);
    let string_word =first_word_slice_general(&s[..]);
    let literal_string_word =first_word_slice_general(x);
    //==Otros sectores (slices)
    //por ejemplo un sector (slice) de una matriz
    let array :[u8; 5] =[1, 3, 5, 6, 7];
    let slice_array :&[u8] =&array[..4];
}
//===funciones de utilidad===
fn first_word_slice_general(param :&str) -> &str
{
    let bytes =param.as_bytes();
    for (i, &element) in bytes.iter().enumerate()
    {
        if element ==b' '
        {
            return &param[0..i];
        }
    }
    &param[..]
}
fn first_word_slice(param :&String) 
    -> &str //de esta forma indicamos un retorno de tipo sector de cadena (slice string)
{
    let bytes =param.as_bytes();
    for (i, &element) in bytes.iter().enumerate()
    {
        if element ==b' '
        {
            return &param[0..i];
        }
    }
    &param[..]
}
fn first_word(param :&String) -> usize
{
    let bytes =param.as_bytes(); //convierte el String en una matriz (array) de bytes para
    //poder iterarlos
    for 
        (i, &character)
    in 
        bytes.iter() 
        .enumerate() //el metodo .enumerate() retornará una tupla con (indice, elemento)
    {
        if character ==b' '
        {//comparamos usando un byte literal b' ' ==32
            return i; //retornamos el  indice en caso de cumplire la condición
        }
    }
    param.len()
}
fn get_string_with_ownership() ->String
{
    let s :String =String::from("dddd");
    s //retorna el String junto con su propiedad
}
/*fn get_reference_string() -> &String
{
    let s :String =String::from("dddd");
    &s //está retornando una referencia a un valor que al finalizar la función saldrá del ámbito
}*/
fn calculate_length_ref_mutable(param : &mut String) -> usize
{
    //se indica & en el parámetro para mostrar que es una referencia y además
    //también se le dá la mutabilidad con la palabra reservada mut
    param.push_str("juesjues");
    param.len()
}
fn calculate_length_ref(param :&String) -> usize
{//la referencia se indica con un "&" en el tipo
//debido a que se trata de una referencia y no hay propiedad sobre ella, no se liberará
//la variable parámetro al salir del ámbito
//como la referencia no es mutable no se puede modificar param
    param.len()
}
fn calculate_length(param :String) -> (String, usize)
{
    let length =param.len();
    (param, length)
}
fn takes_and_gives_back(param :String) -> String
{
    param
}
fn gives_ownership() -> String
{
    let s =String::from("hola"); //definición de un String, dentro del ambito
    s //expresión que retorna la variable anterior
}

fn takes_ownership(param :String)
{
    println!("to {}", param);
}
fn makes_copy(/*mut*/ param :i32)
{
    //param =param*2; //para el caso de parámetro mutable
    //let param =param*2; // para el caso de sombreado
    println!("mc {}", param);
}