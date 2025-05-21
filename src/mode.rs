use dialoguer::Select;

pub fn ask_role() -> bool {
    let options = &["Nodo Maestro", "Nodo Cliente"];

    let selection = Select::new()
        .with_prompt("Selecciona el tipo de nodo")
        .items(options)
        .default(0)
        .interact()
        .unwrap();

    selection == 0 // true si es maestro, false si cliente
}