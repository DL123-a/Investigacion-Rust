struct Persona {
    nombre: String,
    edad: i32,
}

fn main() {
    let mut personas: Vec<Persona> = Vec::new();

    personas.push(Persona {
        nombre: String::from("Daniel"),
        edad: 21,
    });

    personas.push(Persona {
        nombre: String::from("Ana"),
        edad: 22,
    });

    println!("Lista de personas:");

    for persona in personas {
        println!("Nombre: {}, Edad: {}", persona.nombre, persona.edad);
    }
}