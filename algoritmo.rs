// ==========================================
// Ejemplo de concurrencia en Rust
// ==========================================
//
// Este programa demuestra uno de los casos
// de uso más importantes de Rust:
//
// -> Procesamiento concurrente seguro.
//
// Rust es muy eficiente para trabajar con
// múltiples hilos (threads) porque evita
// errores de memoria y data races desde
// compilación.
//
// En este ejemplo:
//
// 1. Creamos un vector de números.
// 2. Dividimos el vector en dos partes.
// 3. Creamos dos hilos.
// 4. Cada hilo suma una parte del vector.
// 5. Finalmente combinamos los resultados.
//
// ==========================================


// Librería necesaria para trabajar con hilos
use std::thread;

fn main() {

    // ==========================================
    // PASO 1: Crear un vector
    // ==========================================
    //
    // Un vector en Rust es una estructura de
    // datos dinámica que puede almacenar varios
    // elementos del mismo tipo.
    //
    // En este caso almacenamos enteros.
    //
    // vec![] es una macro para crear vectores.
    //
    let numeros = vec![1,2,3,4,5,6,7,8,9,10];



    // ==========================================
    // PASO 2: Encontrar la mitad del vector
    // ==========================================
    //
    // len() devuelve la cantidad de elementos.
    //
    // Luego dividimos entre 2 para encontrar
    // la mitad.
    //
    let mitad = numeros.len() / 2;



    // ==========================================
    // PASO 3: Dividir el vector
    // ==========================================
    //
    // Rust permite obtener "slices" del vector.
    //
    // numeros[..mitad]
    // significa:
    // desde el inicio hasta la mitad.
    //
    // numeros[mitad..]
    // significa:
    // desde la mitad hasta el final.
    //
    // to_vec() convierte el slice nuevamente
    // en un vector independiente.
    //
    let parte1 = numeros[..mitad].to_vec();
    let parte2 = numeros[mitad..].to_vec();



    // ==========================================
    // PASO 4: Crear el primer hilo
    // ==========================================
    //
    // thread::spawn() crea un nuevo hilo.
    //
    // move || indica que el hilo tomará
    // ownership de los datos.
    //
    // Ownership es una de las características
    // más importantes de Rust.
    //
    // Gracias a ownership:
    //
    // - No existen data races.
    // - No hay accesos inválidos a memoria.
    // - El compilador verifica la seguridad.
    //
    // iter() recorre los elementos.
    //
    // sum::<i32>()
    // suma todos los enteros del vector.
    //
    let hilo1 = thread::spawn(move || {

        // Suma de la primera mitad
        parte1.iter().sum::<i32>()

    });



    // ==========================================
    // PASO 5: Crear el segundo hilo
    // ==========================================
    //
    // Este hilo trabaja con la segunda mitad
    // del vector.
    //
    let hilo2 = thread::spawn(move || {

        // Suma de la segunda mitad
        parte2.iter().sum::<i32>()

    });



    // ==========================================
    // PASO 6: Esperar resultados
    // ==========================================
    //
    // join() espera a que el hilo termine.
    //
    // unwrap() obtiene el valor resultante.
    //
    // Finalmente sumamos ambos resultados.
    //
    let suma_total =
        hilo1.join().unwrap()
        +
        hilo2.join().unwrap();



    // ==========================================
    // PASO 7: Mostrar resultado
    // ==========================================
    //
    // println! imprime texto en pantalla.
    //
    // {} es un placeholder para variables.
    //
    println!("La suma total es: {}", suma_total);

}



// ==========================================
// ¿Por qué este programa demuestra que
// Rust es eficiente?
// ==========================================
//
// 1. Usa concurrencia real.
//    Los hilos trabajan al mismo tiempo.
//
// 2. Es seguro.
//    Rust evita errores de memoria.
//
// 3. Tiene alto rendimiento.
//    Rust es compilado y muy rápido.
//
// 4. No necesita garbage collector.
//    Esto reduce consumo de memoria.
//
// ==========================================
//
// Comparación:
//
// Python:
// - Más fácil.
// - Pero tiene limitaciones por el GIL.
//
// Java:
// - Buena concurrencia.
// - Usa garbage collector.
//
// Rust:
// - Muy rápido.
// - Seguro.
// - Excelente manejo de concurrencia.
//
// ==========================================