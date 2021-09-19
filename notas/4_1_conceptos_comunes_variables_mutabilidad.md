# Variables y mutabilidad
Las variables por defecto por son inmutables, esto quiere decir que no pueden cambiarde valor, una vez definidas, con la excepción de cuando usamos la palabra reservada "mut" que nos permite prescindir de dicha característica. Las variables se definen con la palabra reservada "let" seguida de su nombre, doble punto ":" y el tipo de la variables, esto último puede ser opcional, en Rust existe la inferencia de tipos.
> lecciones/d_common_concepts/src/main.rs
`
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
    ...
    ...
    ...
`  
## Mutabilidad  
- Variablesinmutables por defecto.
- Si se quiere que una variable sea mutable le debe preceder la palabra reservada "mut".  
## Constantes  
- Las constantes (const) definen variables usando la palabra reversada "const" en lugar de "let".  
- Deben tener un valor no dependiente del tiempo de ejecución.  
- No admiten redefinición (sombreado, shadowing).  
- La convención de nombre es escribir este en mayúsculas.  
## Sombreado  
- Se debe usar la palabra reservada "let" junto al nombre de la variable ya definida.
- Puede cambiar el tipo de la variable ya que se define una variable nueva y se desecha la anterior.

*Plus: practicas/batmam_vs_me_4_1*