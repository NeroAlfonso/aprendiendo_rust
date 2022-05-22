#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
mod frente_a_casa //módulo
{
    mod alojamiento //sub módulo
    {
        fn aniadir_a_la_lista_de_espera(){} //función
        fn sentar_en_la_mesa() {} //función
    }
    mod servicio //sub módulo
    {
        fn tomar_orden() {} //función
        fn servir_orden() {} //función
        fn tomar_pago() {} //función
    }
}
/*
-- arbol del módulo
    crate
    └── frente_a_casa
        ├── alojamiento
        │   ├── aniadir_a_la_lista_de_espera
        │   └── sentar_en_la_mesa
        └── servicio
            ├── tomar_orden
            ├── servir_orden
            └── tomar_pago
*/