use std::collections::HashMap;

#[derive(Debug)]
struct Estudiante {
    nombre: String,
    edad: u32,
    cursos: Vec<String>,
}

fn main() {
    // HashMap para buscar por nombre
    let mut estudiantes: HashMap<String, Estudiante> = HashMap::new();

    // Crear estudiantes
    let e1 = Estudiante {
        nombre: String::from("Ana"),
        edad: 20,
        cursos: vec![String::from("Matemática"), String::from("Rust")],
    };

    let e2 = Estudiante {
        nombre: String::from("Luis"),
        edad: 22,
        cursos: vec![String::from("Algoritmos"), String::from("Bases de Datos")],
    };

    // Insertar en el HashMap
    estudiantes.insert(e1.nombre.clone(), e1);
    estudiantes.insert(e2.nombre.clone(), e2);

    // Mostrar todos los estudiantes
    println!("Lista de estudiantes:");
    for (nombre, estudiante) in &estudiantes {
        println!("Nombre: {}", nombre);
        println!("Edad: {}", estudiante.edad);
        println!("Cursos: {:?}", estudiante.cursos);
        println!("-------------------");
    }

    // Buscar un estudiante
    let nombre_busqueda = "Ana";
    match estudiantes.get(nombre_busqueda) {
        Some(est) => println!("Encontrado: {:?}", est),
        None => println!("No encontrado"),
    }
}