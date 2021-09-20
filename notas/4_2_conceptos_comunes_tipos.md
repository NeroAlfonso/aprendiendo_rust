# Tipos de datos  
Cada valor en rust tiene un tipo, existen dos tipos de datos, escalares y compuestos.  
> lecciones/d_common_concepts/src/main.rs
`
    ...
    ...
     //===tipos de datos===
        //==Escalares
            let integer :u8 =10;
        //=punto flotante o decimales
            let v =2.2; //f64
            let v :f32 =v;
            println!("decimal: {}", v);
        //=booleanos
            let success =true;
            let success :bool =false;
        //caracter
            let character :char ='S';
    //===tipos compuestos===
        //==tipo tupla
        let tuple : (u32, char, f32) =(1, 'd', 0.2);
        //destructuración
        let (a, b, c) =tuple;
        println!("variable a: {}", a);
        //indices
        println!("indice uno: {}", tuple.1);
    //==tipo arreglo
        let array =[1, 2, 3, 4];
        let array :[char; 5] =['a', 'b', 'c', 'd', 'e'];
        let array :[f32; 10] =[0.1; 10];
        println!("indice cero: {}", array[0]);
        println!("indice uno: {}", array[1]);
    ...
    ...
`  
## Escalares  
Son valores simples, rust cuenta con 4 principales.
### Enteros  
- Existen con signo (i) y sin signo (u), pueden ser de 8, 16, 32, 64 y 128 bits.
- Sus valores van de -(2^n-1) to (2^n-1)-1 donde n es el numero de bits cuando son con signo (in) y de cero a (2^n-1)-1 donde n es el numero de bits cuando son sin signo (un).
- El tipo size (usize, isize) adoptará el tipo de arquitectura del sistema (64 o 32 bits)
- Por defecto rust usa i32 (32 bits con signo) pero para indexar una colección prefiere los tipos size (usize , isize).
- Los enteros literales se pueden escribir usando el separador "_" (decimal).
- rust en modo release maneja el desbordamiento de enteros, aunque también hay forma de manejarlo.  
`
let integer :u8 =10;            
`
### Decimales              
- Existen el tipo de 32 bits (f32) y el de 64 bits (f64) (mayor precisión).
`
let v =2.2; //f64
let v :f32 =v;
`
### Booleanos  
Representan un valor binario.  
`
    let success : bool =true;
`  
### Caracteres
Se especifican con comillas simples (distinto a las cadenas literales).  
`
    let character :char ='S';
`  
## Compuestos  
Los tipos compuestos son tipos de datos que contienen multiples valores. Rust cuenta con dos tipos compuestos primitivos, las tuplas (tuples) y los arreglos (arrays).  
### Tuplas 
- Se definen como *let tuple : (tipos...) = (valores...);*
`
    let tuple : (u32, char, f32) =(1, 'd', 0.2);
`
- Admiten destructuración.  
`
    let (a, b, c) =tuple;
`
- Están indexados.  
`
    println!("indice uno: {}", tuple.1);
`
### Arreglos  
- Son de extensión y tipo fijo (un solo tipo por matriz).  
`
    let array =[1, 2, 3, 4];
    let array :[char; 5] =['a', 'b', 'c', 'd', 'e'];
`  
- Tiene algunos atajos para su definición, por ejemplo a continuación se define un arreglo de n posiciones con el mismo valor.  
`
    let array :[f32; 10] =[0.1; 10];
`  
- Acceder al indice.  
`
    println!("indice cero: {}", array[0]);
    println!("indice uno: {}", array[1]);
`  
- Entra en panico si se trata de acceder un indice que no existe.  
*Plus: practicas/batmam_vs_me_4_2*  