use std::io;

// Definicion de una estructura (struct) llamada Tarea
// Representa una tarea con una descripcion y su estado (completada o no)
struct Tarea {
    descripcion: String,
    completada: bool,
}

// Funcion para agregar una nueva tarea al vector
// Recibe una referencia mutable (&mut) porque modifica la lista
fn agregar_tarea(lista: &mut Vec<Tarea>, descripcion: String) {
    let nueva = Tarea {
        descripcion,
        completada: false,
    };

    // push agrega un elemento al final del vector
    lista.push(nueva);
}

// Funcion para mostrar todas las tareas
// Recibe referencia inmutable (&) porque solo lee datos
fn mostrar_tareas(lista: &Vec<Tarea>) {

    // iter() recorre el vector
    // enumerate() agrega el indice a cada elemento
    for (i, tarea) in lista.iter().enumerate() {

        // if como expresion para determinar el estado visual
        let estado = if tarea.completada { "completada" } else { "pendiente" };

        println!("{}: {} [{}]", i, tarea.descripcion, estado);
    }
}

// Funcion para marcar una tarea como completada
// Usa get_mut para obtener una referencia mutable segura
fn completar_tarea(lista: &mut Vec<Tarea>, indice: usize) {

    // get_mut devuelve un Option
    // Some si el indice existe, None si no existe
    if let Some(tarea) = lista.get_mut(indice) {

        // Se modifica directamente el campo completada
        tarea.completada = true;

    } else {
        println!("Indice invalido");
    }
}

fn main() {

    // Se crea un vector vacio que almacenara las tareas
    let mut lista: Vec<Tarea> = Vec::new();

    // loop crea un ciclo infinito hasta que se use break
    loop {

        println!("\n1. Agregar tarea");
        println!("2. Mostrar tareas");
        println!("3. Completar tarea");
        println!("4. Salir");

        // Variable para guardar la opcion del usuario
        let mut opcion = String::new();

        // read_line lee entrada del usuario desde consola
        io::stdin().read_line(&mut opcion).unwrap();

        // match es similar a un switch
        // Se usa para decidir que hacer segun la opcion ingresada
        match opcion.trim() {

            "1" => {
                println!("Ingrese descripcion:");

                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();

                // Se envia la lista como mutable porque se modificara
                agregar_tarea(&mut lista, desc.trim().to_string());
            }

            "2" => {
                // Solo se pasa referencia inmutable porque no se modifica
                mostrar_tareas(&lista);
            }

            "3" => {
                println!("Ingrese indice:");

                let mut idx = String::new();
                io::stdin().read_line(&mut idx).unwrap();

                // parse convierte string a numero
                // unwrap_or evita error si el valor no es valido
                let i: usize = idx.trim().parse().unwrap_or(999);

                completar_tarea(&mut lista, i);
            }

            "4" => break, // break termina el loop

            _ => println!("Opcion invalida"),
        }
    }
}