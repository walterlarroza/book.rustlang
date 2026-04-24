fn main() {

    let mut intentos = 0; //variable mutable para el contador de intentos

    const CLAVE_MAESTRA: i32 = 1234; //definimos una constaste que es la correcta

    println!("Sistema de seguridad Iniciado");

    loop{
        intentos = intentos + 1; //shadowing o actualizacion de variable mut

        println!("Intento numero: {}", intentos);
       
        let resultado = validar_acceso(1234, CLAVE_MAESTRA); //llama funcion con retorno

        if resultado{
            println!("Acceso concedido en el intento {}", intentos);
            break; //salida mandatori del bucle
        }

        if intentos == 3 {
            println!("Sistema Bloqueado: Demanciados intentos");
            break;
        }
    }
}

//funcion con parametros y tipo de retorno booleano
fn validar_acceso(intento: i32, real: i32) -> bool {
    // la ultima linea sin ";" es lo que se retorna
    intento == real
} 	       

