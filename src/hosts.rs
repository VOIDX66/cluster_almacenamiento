use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use dialoguer::{Input, Confirm, theme::ColorfulTheme};

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
    let mut existing_lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    loop {
        println!("\n📄 Entradas actuales en /etc/hosts:");
        for line in &existing_lines {
            println!("{}", line);
        }

        let ip: String = Input::with_theme(&theme)
            .with_prompt("Ingresa la IP del nodo")
            .interact_text()
            .unwrap();

        let hostname: String = Input::with_theme(&theme)
            .with_prompt("Ingresa el nombre del nodo (ej: vm1)")
            .interact_text()
            .unwrap();

        let new_entry = format!("{} {}", ip.trim(), hostname.trim());

        if existing_lines.iter().any(|line| line.trim() == new_entry) {
            println!("⚠️  Esa entrada ya existe.");
        } else {
            if Confirm::with_theme(&theme)
                .with_prompt(format!("¿Agregar '{}' al archivo?", new_entry))
                .interact()
                .unwrap()
            {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(path)
                    .expect("❌ No se pudo abrir el archivo para escritura");

                writeln!(file, "{}", new_entry).expect("❌ No se pudo escribir en /etc/hosts");
                println!("✅ Entrada agregada correctamente.");
                existing_lines.push(new_entry);
            }
        }

        let again = Confirm::with_theme(&theme)
            .with_prompt("¿Quieres agregar otra entrada?")
            .interact()
            .unwrap();

        if !again {
            break;
        }
    }
}
