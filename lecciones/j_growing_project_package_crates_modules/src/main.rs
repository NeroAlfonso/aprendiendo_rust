fn main()
{
    println!("Proyectos crecientes, paquetes, crates y modulos");
    //----------Packages and crates (paquetes y cajas)----------
    //un paquete puede ser creado con la instrucción "cargo create project_name"
    //y contiene binarios y librerias (crates). Al ejecutar la instrucción
    //esto creará un archivo .toml que contiene la especificación del paquete
    //incluyendo como se construirán (build) estos crates
    //Cargo.toml
    //este archivo no contiene referencias a main.rs porque Cargo sigue la convención
    //de que el src/main.rs es la raíz del crate binario con el mismo nombre del paquete
    //igualmente Cargo reconoce que si existe en src/lib.rs entonces el paquete contiene
    //una librería crate con el mismo nombre del paquete
    //Cargo pasa estos archivos raiz a rustc (compilador) para que construya (build)
    //los binarios o librerías
    //inicialmente solo tenemos src/main.rs, un crate binario llamado igual que el paquete
    //en caso de tener un src/lib.rs tendriamos dos crates, una librería (lib.rs) y un binario 
    //(main.rs) ambos con el mismo nombre del paquete (package)
    //un paquete puede tener multiples crates binarios, cada uno como un archivo en src/bin
    //Por ejemplo tenemos el crate rand usado para generar un número aleatorio (empleado en el "guessing game")
    //c_guessing_game. Si definimos un struct llamado "Rng" (igual que el rasgo (trait) Rng provisto por el crate
    //rand), e importamos el crate rand, no habría conflictos debido a que de querer acceder al
    //rasgo (trait) Rng deberiamos hacerlo de la siguiente manera rand::Rng y en cuanto a la estructura (struct)
    //podemos hacerlo como Rng (espacio de nombre)
    //----------Definiendo modulos para controlar el ambito y la privacidad----------
    
}
