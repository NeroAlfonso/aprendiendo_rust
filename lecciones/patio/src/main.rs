use crate::jardin::vegetales::Esparragos;

pub mod jardin;

fn main() {
    let planta = Esparragos {};
    println!("Estoy cultivando {:?}!", planta);
}