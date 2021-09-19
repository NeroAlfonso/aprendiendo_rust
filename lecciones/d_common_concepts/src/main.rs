fn main() {
    //===mutabilidad===
    let x =5; //let x : u32 =5;
    //las variables en rust son inmutables por defecto
    //x =6; //esto ocasionaría un error
    let mut y =8; //especificando la mutabilidad de la variable, es posible reasignarle valores
    println!("{}", y);
    y =10;
    println!("{}", y);
    //===constantes===
    //las constantes siempre son inmutables (no admite sombreado)
    //siempre se le debe especificar el tipo de datos
    //su valor debe venir de una fuente constante y no como resultado de una 
    //función o dependiente del tiempo de ejecución
    //se pueden declarar en cualquier ambito
    //convencion de nombre en mayusculas
    const Z :u32 =20;
    println!("{}", Z);
    //===sombreado===
    //se debe usar la palabra reservada let y el mismo nombre de la variable a sombrear
    //puedes cambiar el tipo ya que se crear una nueva variable desechando la anterior
    let w =5; //sombreada
    println!("inicial {}", w);
    let w =w+1; //sombreada
    println!("mas uno {}", w);
    let x =w*2;
    println!("el doble {}", w);
    let spaces ="   ";
    println!("espacios '{}'", spaces);
    let spaces =spaces.len();
    println!("tamaño {}", spaces);
    //===tipos de datos===
        //==Escalares
        //son enteros, decimales, booleanos y caracteres
            //=enteros
            //existen con signo y sin signo, de 8, 16, 32, 64, 128 bits
            //sus valores van de -(2^n-1) to (2^n-1)-1 donde n es el numero de bits cuando son con signo (in)
            //y de cero a (2^n-1)-1 donde n es el numero de bits cuando son sin signo (un)
            //el tipo size (usize, isize) adoptará el tipo de arquitectura del sistema (64 o 32 bits)
            //por defecto rust usa i32 (32 bits con signo) pero para indexar una colección prefiere los tipos size (usize , isize)
            //los enteros literales se pueden escribir usando el separador "_" (decimal)
            //rust en modo release maneja el desbordamiento de enteros, aunque también hay forma de manejarlo
            let integer :u8 =10;
            //=punto flotante o decimales
            //existen el tipo de 32 bits (f32) y el de 64 bits (f64) (mayor precisión)
            let v =2.2; //f64
            let v :f32 =v;
            println!("decimal: {}", v);
            //=booleanos
            let success =true;
            let success :bool =false;
            //caracter
            //se especifican con comillas simples (distinto a las cadenas literales)
            let character :char ='S';
    //===tipos compuestos===
        //==tipo tupla
        //(tipos...) = (valores...)
        let tuple : (u32, char, f32) =(1, 'd', 0.2);
        //destructuración
        let (a, b, c) =tuple;
        println!("variable a: {}", a);
        //indices
        println!("indice uno: {}", tuple.1);
        //==tipo arreglo
        //son de extensión y tipo fijo (un solo tipo por matriz)
        let array =[1, 2, 3, 4];
        let array :[char; 5] =['a', 'b', 'c', 'd', 'e'];
        //llenar un arreglo n veces con el mismo valor
        let array :[f32; 10] =[0.1; 10];
        //acceder al indice
        println!("indice cero: {}", array[0]);
        println!("indice uno: {}", array[1]);
        //entra en panico si se trata de acceder un indice que no existe
    //===operadores numericos===
    //suma, resta, producto, división y residuo (modulo)
    let calc =5+5;
    println!("suma: {}", calc);
    let calc =calc -5;
    println!("resta: {}", calc);
    let calc =calc *2;
    println!("producto: {}", calc);
    let calc =calc /2;
    println!("división: {}", calc);
    let calc =calc %2;
    println!("residuo: {}", calc);
}   
