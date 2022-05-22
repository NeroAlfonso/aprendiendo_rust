# Propiedad  
- Es la forma en la que rust maneja la memoria de manera segura
## Pila (stack) y monton (heap)  
Ambas son formas en las que la memoria almacena valores y los pone a disposición de nuestro programa, lo que los diferencia principalmente es la forma en la que estos datos estan almacenados  
### Pila (stack)  
- Registra sus valores en el orden en que son suministrados y los sustrae en el orden contrario (primero que entra ultimo que sale)
- Agregar valores a la pila se denomina "pushing onto the stack"
- Remover valores de la pila se denomina "popping off the stack"
- Todos sus valores deben ser conocidos y de una longitud fija  
- El orden de los valores facilita la lectura de los mismos  
### Monton (heap)  
- Valores desconocidos en tiempo de compilación o que pueden variar
- Menor organización
- Poner valores en el montón se denomina "allocating in the heap" o "allocating" y requiere la ubicación de un espacio suficientemente grande para almacenar los valores
- La necesidad de determinar un espacio disponible en la memoria para almacenar los valores y de igual manera ubicar los valores para lectura, exigen mucho mas computo que operar sobre la pila  

Cuando una función es llamada, tanto los parámetros suministrados, como las variables locales de la función son agregadas (pushed onto the stack) a la pila, el terminar esta, dichos valores son removidos (popped off the stack) de la pila  

La propiedad (ownership) por medio de estos mecanismos, se asegura de minimizar datos duplicados en el montón y limpiar este de los valores que están en desuso

## Reglas de la propiedad

- Cada valor en Rust asignado a una variable tiene un propietario (owner), que es la variable misma  
- Cada valor solo puede tener un propietario a la vez  
- Cuando el propietario (owner) sale fuera del alcance o contexto (scope), el valor es abandonado (dropped)  
## Alcance de las variables  
- El alcance de las variables está determinado por el bloque de ejecución  
`
 { //s no es valido
     let s ="hola";
     //s es valido
 } //fuera del alcance. s no es valido
`  
## Entendiendo la propiedad con el tipo String  
El tipo string se almacena en el montón (heap), tiene un valor que puede mutar en tiempo de ejecución, como se puede apreciar a continuación  
`
 let mut s = String::from("hola");
 s.push_str(", mundo");
 //"hola, mundo"
`  
### Memoria y asignación (allocation)  

> archivo
`
Código
`  
Contenido.
- Lista.
## Sub-título
`
Código
`  
*Nota*