fn main()
{
    println!("Proyectos crecientes, paquetes, crates y modulos");
    /*
        El sistema de módulos de Rust (rust module system), consta de:
        - paquetes (packages): una caracteristica del manejador de paquetes cargo, que te permite construir, 
        testear y compartir crates
        - crates : un arbol de modulos que genera una librería o un ejecutable
        - módulos (modules / use): te dá el control de la organización, el contexto (scope) y las rutas (paths)
        - rutas (paths): una forma de nombrar un elemento, como una estructura, una función o un módulo
    */
    //----------Packages and crates (paquetes y cajas)----------
    /*
            Un paquete (package) contiene uno o mas crates que proveen un conjunto de funcionalidades, un paquete 
        contiene un archivo cargo.toml que especifica como se construiran estos crates.
            Un crate puede ser de tipo binario (binary) o libreria (library).
            - Binario (binary): Estos crates pueden ser compiladores para crear ejecutables. Estos deben tener 
            una función llamada "main()", que especifica que sucede cuando el ejecutable inicia su funcionamiento
            - Librería / biblioteca (library): Estos crates no tiene una función principal "main()", y no se compilan
            en ejecutables. Su fin ultimo es especificar funcionalidades para ser compartidas entre otros proyectos. 
            Por ejemplo el crate rand (c_guessing_game).
            La raíz del crate (root) es un archivo fuente a partir de donde el compilador rust se inicia y constituye
            el módulo raíz del crate (ya se ahondará en el tema de módulos)
            
            Contenido de un paquete (package):
            - Solo un crate librería / biblioteca (library crate)
            - No hay un límite para los crates binarios (binary crate) que pueda contener
            - Debe contener al menos un crate, binario o librería / biblioteca

            Generando un nuevo paquete:
            - cargo new j_growing_project_package_crates_modules: Cargo creará un archivo cargo.toml proveyéndonos de
            un nuevo paquete
            Inspeccionando el archivo cargo.toml generado, no existe una referencia a src/main.rs porque cargo sigue
            convencionalmente la ruta src/main.rs como la raíz (root) del crate binario (binary crate), con el mismo
            nombre del paquete.
            De igual manera, cargo entiende, que si en src/ encuentra lib.rs (src/lib.rs), el paquete contiene un crate
            librería / biblioteca (library crate), con el mismo nombre del paquete y como raíz del crate (crate root) el
            archivo src/lib.rs
            Cargo pasa los archivo raíz al compilador de rust (rustc) para construit la libreria / biblioteca (library) o
            el binario (binary)
            Si el paquete contiene src/main.rs y src/lib.rs entonces contiene dos crates, uno binario y uno libreria / biblioteca,
            ambos con el mismo nombre del paquete. Un paquete puede contener multiples crates binarios con sus archivos colocados
            en el directorio src/bin, cada archivo será un crate binario independiente
    */
    //----------Defining modules to control scope and privacy----------
    /*
        - referencia rápida de módulos (como trabaja el compilador)
            - Iniciando por la raíz del crate: Cuando compilamos un crate, el compilador primero buscará en los
            archivos raíz del crate (tipicamente src/main.rs para crates binarios o src/lib.rs para crates librería
            (biblioteca))
            
            - Declarando módulos: el el archivo raíz del crate, podemos declarar un nuevo módulo, llamado para
            efectos de este ejemplo "jardin" haciendo uso de las palabras reservada "mod" de la siguiente manera
            "mod jardín;". El compilador buscará los archivos del modulo en cuestión, dentro de los siguientes lugares:
                - en linea (inline): Inmediatamente después de "mod jardín" en lugar de un punto y coma tendríamos unas
                llaves "{...}" con su especificación en medio.bool
                - en archivo: src/jardín.rs o en src/jardín/mod.rs
            
            - declarando sub-módulos: el cualquier otro archivo que se esté compilando y que no sea la raíz del crate
            pero que se esté compilando como parte del crate, se pueden declarar sub-módulos por ejemplo uno llamado
            "vegetales" como "mod vegetales;". El compilador buscará el código de este sub-módulo en los siguientes
            lugares:
                - en linea (inline): inmendiatamente después de "mod vegetales" en lugar de tener un punto y coma
                tendría un par de llaves "{...}" con la especificación del sub-módulo entre ellas.
                - en archivo: src/jardín/vegetales.rs o src/jaardín/vegetales/mod.rs
            
            - rutas a código en los módulos: Cuando un módulo se está compilando como parte de un crate, puedes
            hacer referencia a su código (por ejemplo a un tipo "Espárragos" dentro del sub-módulo "vegetales") desde 
            cualquier lugar del crate en cuestión usando la ruta (a efectos de este ejemplo) 
            crate::jardín::vegetales::Espárragos, siempre que las reglas de privacidad lo permitan
            
            - privado o público: el código de un módulo es por defecto privado para sus módulos principales. Para
            hacer un módulo público declaralo con la palabra reservada "pub" de la sigueinte manera "pub mod nombredelmódulo"
            en lugar de "mod nombredelmódulo". Para hacer públicos los elementos dentro de un módulo público se usa la
            misma palabra reservada "pub" antes de su declaración.

            - la palabra reservada "use": dentro de un ámbito se pueden crear atajos a items con rutas largas,
            para evitar la repetición. El cualquier ámbito al que puede referirse crate::jardín::vegetales::Espárragos
            podemos crear un atajo con "use crate::jardín::vegetales::Espárragos;" y luego solo debemos escribir "Espárragos"
            para hacer uso de este tipo en el ámbito o contexto.

        Ilustramos este ejemplo: Un crate binario llamado "patio", el directorio del crate también es llamado
        "patio" y contiene estos archivos
            patio
            ├── Cargo.lock
            ├── Cargo.toml
            └── src
                ├── jardín
                │   └── vegetales.rs
                ├── jardín.rs
                └── main.rs
        *ejemplo en lecciones/patio*
    */
    //----------Grouping related code in modules----------
    /*
        Crearemos un crate de tipo librería / biblioteca para ilustrar el uso de los módulos.
        Nuestra presima es modelar un restaurante, donde tenemos partes definidas, como frontal (meseros,
        clientes) y trasera (cocineros, lavaplatos).
        *ejemplo en lecciones/restaurante*
        lo creamos ejecutando "cargo new --lib restaurante"
    */
}

