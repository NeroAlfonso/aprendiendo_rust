#[derive(Debug)] //agregando esta anotación jussto antes de la definición de la estructura (struct)
//le damos el rasgo (trait) Debug a la misma, para que pueda formatearse en caso de querer imprimirla
struct Rectangle 
{
    width: u32,
    height: u32
}
//implementación de método
impl Rectangle //con la palabra impl (implementation) seguida del nombre de la estructura (struct)
//vamos a agregarle un método a la misma
//los bloques impl de las estructuras pueden ser multiples, o como en este caso, solo uno que contiene
//todas las definiciones
{//escribimos la función y asignamos el primer parámetro a self
    fn area(&self) -> u32 //el parámetro self se refiere a la instancia en si misma, este parámetro
    //no debe ser suministrado para funcionar, en este caso una referencia a la instancia en si misma
    //en este caso no queremos editar el valor de la instancia, de quererlo el parámetro debería ser
    //mutable (&mut self), de lo contrario el método tomaría la propiedad de la instancia
    //ceder la propiedad de self al método no es muy usual pero suele usarse en casos en los que se
    //quiere modificar la instancia y evitar que se pueda acceder al estado original de la instancia
    {
        self.width * self.height
    }
    fn can_hold(&self, other : &Rectangle) -> bool
    {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) ->Rectangle //funcioón asociada (no método, ya que no tiene una referencia 
    //a la instancia misma)
    {
        Rectangle{
            width: size,
            height: size
        }
    }
}
struct User //para definirlos usa la palabra reervada struct seguido del nombre de la estructura
//y dentro del cuerpo el detalle de sus campos (fields), junto a su tipo
{
    username: String,
    email: String, 
    sign_in_count: u64,
    active: bool
} //distinto de las tuplas (tuples), no se depende del orden de los datos para acceder a ellos
struct Color(i32, i32, i32); //la tupla estructura (structure tuple) es como una tupla regular solo que nombrada
//instanciable y nos permite usarla como un tipo
fn main() {
    //las estructuras son similares a los objetos en POO
    let user1 : User = User
    {
        username: String::from("nerioperdomo"),
        email: String::from("nerioperdomo28@gmail.com"),
        active: true,
        sign_in_count: 1
    }; //instancia de User
    println!("user 1 email {}", user1.email);
    //user1.active =false; //esta declaración provocaría un error, debido a que la instancia
    //no es mutable
    let mut user2 =user1;
    //println!("user 1 email {}", user1.email); //esta declaración provocaría un error, debido
    //a que la propiedad de user1 ha sido suministrada a user2
    user2.email =String::from("neroperdomo@gmail.com"); //ya está permitido, debido a que esta nueva
    //variable es mutable (mut)
    println!("user 2 email {}", user2.email);
    //la mutabilidad sobre las estructuras (structs) es general, no puede aplicarse a solo algunos campos
    //también podemos usar una función como generador de usuarios
    let user3 =user_builder(String::from("Juan José"), String::from("juan@hotmail.com"));
    println!("user 3 email {}", user3.email);
    //aplicación de la taquigrafía en la instanciación de un struct (dentor de la función)
    let user4 =user_builder_shorthand(String::from("José Juan"), String::from("josé@hotmail.com"));
    println!("user 4 email {}", user4.email);
    //para crear o actualizar una estructura (struct) basandote en otra estructura puede hacerlo
    //como es previsible
    let user5 =User{
        username: String::from("Mariana"),
        email: String::from("Mariana@hotmail.com"),
        active: user4.active,
        sign_in_count: user4.sign_in_count
    };
    //o tambien puedes usar la sintaxis de actualización, para asignar los valores no suministrados
    //a la instancia a partir de otra instancia
    let user5 =User{
        username: String::from("Mariana"),
        email: String::from("Mariana@hotmail.com"),
        ..user4 //..instancia sería la sintaxis, asignaría los atributos active y sign_in_count
        //de user4
    };
    //==estructuras tupla (tuple structure)
    let black :Color =Color(0, 0, 0); //instanciando una estructura tupla (tuple struct)
    //se puede usar la sintaxis black.0, 1, 2 para acceder a los datos como una tupla regular
    //==estructuras unidad (sin campos)
    //ahondado en traits
    //==propiedad, referencias en estructuras de datos
    //ahondado en lifetimes
    struct_example();
    //==metodos
    methods();
}
fn methods()
{
    println!("metodos");
    let rect : Rectangle =Rectangle{
        width: 30, 
        height: 50
    };
    println!("el area del rectangulo es {}", rect.area()); //usamos el metodo de la estructura (struct)
    //Rectangle (area) para calcular el area
}
fn struct_example()
{
    rectangle_area_variables();
    rectangle_area_tuples();
    rectangle_area_structs(); //este  ejemplo añade todo el significado y deja poco a la imaginación,
    //la estructura deja claro los conceptos y la dinamica del código, ademas de minimizar los errores
    //dandole mas sentido al código
}
fn rectangle_area_structs()
{
    println!("estructura");
    let rect : Rectangle =Rectangle{
        width: 30, 
        height: 50
    };
    println!("el area del rectangulo es {}", area_struct(&rect));
    println!("el rectangulo es {:?}", rect); //:? corresponde al formateador de depuración, que 
    //nos permitirá ver objetos complejos con println!, para esto la estructura (struct) en cuestión debe
    //implementar el rasgo (trait) Debug
    println!("el rectangulo es {:#?}", rect); //agregando :#? agregará saltos de línea para mejorar
    //la lectura
    //probemos el nuevo método implementado can_hold con parámetros
    let rect2 : Rectangle =Rectangle{
        width: 20, 
        height: 40
    };
    let rect3 : Rectangle =Rectangle{
        width: 40, 
        height: 50
    };
    let res :bool = rect.can_hold(&rect3);
    println!("rect entra en {}", res);
    //==funciones asociadas
    //son funciones que pueden implementarse en la estructura (struct) y no tienen como parametro
    //la instancia en si misma (self)
    let sq :Rectangle =Rectangle::square(10); //se usa la sintaxis :: para llamar a la función asociada
}
fn area_struct(rect : &Rectangle) -> u32
{
    rect.width * rect.height
}
fn rectangle_area_tuples()
{
    println!("tupla");
    let rect : (u32, u32) =(30, 50);
    println!("el area del rectangulo es {}", area_tuple(rect));
}
fn area_tuple(rect : (u32, u32)) -> u32
{
    rect.0 * rect.1
}
fn rectangle_area_variables()
{
    println!("variables");
    let width =30;
    let height =50;
    println!("el area del rectangulo es {}", area(width, height));
}
fn area(width :u32, height :u32) -> u32
{
    width * height
}
fn user_builder_shorthand(username :String, email :String) -> User
{//podemos usar la taquigrafía para asociar a un struct sus valores si estos tienen el mismo nombre
    //que los parámetros con los valores afines
    User
    {
        username,
        email,
        active:         true,
        sign_in_count:  1
    }
}
fn user_builder(username :String, email :String) -> User
{
    User
    {
        username:       username,
        email:          email,
        active:         true,
        sign_in_count:  1
    }
}