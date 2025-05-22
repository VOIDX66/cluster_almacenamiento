use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use dialoguer::{Input, Select, Confirm, theme::ColorfulTheme};

pub fn edit_hosts() {
    let theme = ColorfulTheme::default();
    let path = "/etc/hosts";

    // Leer contenido actual
    let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("❌ No se pudo abrir /etc/hosts. ¿Tienes permisos de sudo?");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut entries: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    loop {
        println!("\n📄 Entradas actuales en /etc/hosts:");

        for (i, line) in entries.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }

        let options = vec![
            "➕ Agregar nueva entrada",
            "✏️ Modificar entrada existente",
            "🗑️ Eliminar entrada existente",
            "💾 Guardar y salir",
            "❌ Salir sin guardar",
        ];

        let choice = Select::with_theme(&theme)
            .with_prompt("¿Qué quieres hacer?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match choice {
            0 => { // Agregar
                let ip: String = Input::with_theme(&theme)
                    .with_prompt("Ingresa la IP del nodo")
                    .interact_text()
                    .unwrap();

                let hostname: String = Input::with_theme(&theme)
                    .with_prompt("Ingresa el nombre del nodo (ej: vm1)")
                    .interact_text()
                    .unwrap();

                let new_entry = format!("{} {}", ip.trim(), hostname.trim());

                if entries.iter().any(|line| line.trim() == new_entry) {
                    println!("⚠️  Esa entrada ya existe.");
                } else {
                    entries.push(new_entry);
                    println!("✅ Entrada agregada.");
                }
            }
            1 => { // Modificar
                if entries.is_empty() {
                    println!("⚠️ No hay entradas para modificar.");
                    continue;
                }

                let selection = Select::with_theme(&theme)
                    .with_prompt("Selecciona la entrada a modificar")
                    .items(&entries)
                    .default(0)
                    .interact()
                    .unwrap();

                let ip: String = Input::with_theme(&theme)
                    .with_prompt("Nueva IP")
                    .with_initial_text(
                        entries[selection]
                            .split_whitespace()
                            .next()
                            .unwrap_or_default(),
                    )
                    .interact_text()
                    .unwrap();

                let hostname: String = Input::with_theme(&theme)
                    .with_prompt("Nuevo nombre de nodo")
                    .with_initial_text(
                        entries[selection]
                            .split_whitespace()
                            .skip(1)
                            .collect::<Vec<_>>()
                            .join(" "),
                    )
                    .interact_text()
                    .unwrap();

                entries[selection] = format!("{} {}", ip.trim(), hostname.trim());
                println!("✅ Entrada modificada.");
            }
            2 => { // Eliminar
                if entries.is_empty() {
                    println!("⚠️ No hay entradas para eliminar.");
                    continue;
                }

                let selection = Select::with_theme(&theme)
                    .with_prompt("Selecciona la entrada a eliminar")
                    .items(&entries)
                    .default(0)
                    .interact()
                    .unwrap();

                if Confirm::with_theme(&theme)
                    .with_prompt(format!("¿Eliminar '{}'? Esta acción es irreversible.", entries[selection]))
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    entries.remove(selection);
                    println!("✅ Entrada eliminada.");
                }
            }
            3 => { // Guardar y salir
                let mut file = match OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(path)
                {
                    Ok(f) => f,
                    Err(_) => {
                        eprintln!("❌ No se pudo abrir /etc/hosts para guardar. ¿Tienes permisos de sudo?");
                        return;
                    }
                };

                for line in &entries {
                    if let Err(e) = writeln!(file, "{}", line) {
                        eprintln!("❌ Error escribiendo en archivo: {}", e);
                        return;
                    }
                }
                println!("💾 Cambios guardados correctamente.");
                break;
            }
            4 => { // Salir sin guardar
                if Confirm::with_theme(&theme)
                    .with_prompt("¿Seguro que quieres salir sin guardar?")
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    println!("❌ Cambios descartados.");
                    break;
                }
            }
            _ => {}
        }
    }
}
