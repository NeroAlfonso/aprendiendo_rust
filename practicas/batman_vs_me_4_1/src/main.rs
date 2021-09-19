use std::io;
use rand::Rng;
struct Attack
{
    command:        u8,
    description :   String,
    damage :        i16
}
fn main()
{
    println!("================= Bienvenido a batman vs me =================");
    let mut batman_hp : i16 =200;
    let mut me_hp : i16 = 100;
    let me_attacks  :[Attack; 3 ] = [
        Attack{
            description: String::from("Patada"),
            damage: 20,
            command: 1
        },
        Attack{
            description: String::from("Puñetazo"),
            damage: 10,
            command: 2
        },
        Attack{
            description: String::from("Susurrar 'Marta'"),
            damage: 30,
            command: 3
        }
    ];
    let mut me_last_attack : Option<&Attack> =None; //definida
    loop
    {
        println!("Batman {} <3", batman_hp);
        println!("Tu {} <3", me_hp);
        if batman_hp <=0 && me_hp <=0
        {
            println!("¡Empate¡ Gordón viene en camino, has perdido");
            break;
        }
        else if batman_hp <=0 {
            println!("¡Batman ha caido!");
            break;
        }
        else if me_hp <=0 {
            println!("¡Has caido!");
            break;
        }
        println!("============================");
        
        for attack in me_attacks.iter()
        {
            if attack.command != match me_last_attack {
                Some(attack) => attack.command,
                None => 0
            }
            {
                println!("{}. {} ({} PA)", attack.command, attack.description, attack.damage);
            }
        }
        println!("======elige un ataque=======");
        let mut attack_input :String =String::new(); //redefinida
        io::stdin().read_line(&mut attack_input).expect("Error en el comando");
        let attack_input :u8 = match attack_input.trim().parse()
        {
           Ok(num) => num,
           Err(_) => {
                clear_screen();
                println!("¡Has fallado el ataque!");
                me_hp =batman_attack(me_hp);
                continue;
            }
        };
        let mut index : usize =me_attacks.len();
        let attack_input : Option<&Attack> = loop { //redefinida
            let att : &Attack = &me_attacks[(index-1)];
            if att.command ==attack_input
            {
                break Some(att);
            }
            index -=1;            
            if index ==0 {break None}
        };
        clear_screen();
        me_hp =batman_attack(me_hp);
        if !attack_input.is_some()
        {
            println!("¡Has fallado el ataque!");
            continue;
        }
        else if 
            (attack_input.is_some() && me_last_attack.is_some())
            &&
            (attack_input.unwrap().command == me_last_attack.unwrap().command)
        {
            println!("¡Has fallado el ataque!");
            continue;
        }
        else
        {
            println!("¡Has propinado un/a {} a Batman! (-{}HP)", attack_input.unwrap().description, attack_input.unwrap().damage);
            batman_hp -=attack_input.unwrap().damage;
            me_last_attack =attack_input;
        }
        continue;
    }
    fn clear_screen()
    {
        print!("\x1B[2J\x1B[1;1H");
    }
    fn batman_attack(mut me_hp: i16) -> i16
    {
        const PROBABILITY : i16= 20; //valores constantes
        let critical :i16 =rand::thread_rng().gen_range(1..100);
        let mut multiplier : i16 =1;
        const DAMAGE : i16 =5; //valores constantes
        if critical <=PROBABILITY
        {
            multiplier =8;
        }
        me_hp -=DAMAGE*multiplier;
        println!("¡Batman te ha guiñado el ojo! (-{}HP)", DAMAGE*multiplier);
        return me_hp;
    }
}
