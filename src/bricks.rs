use std::{fs, path::Path, process::Command};
use dialoguer::{theme::ColorfulTheme, Input, Select, Confirm};

fn create_brick() {
    let brick_name: String = Input::new()
        .with_prompt("Nombre del brick (ej. datos, respaldo)")
        .interact_text()
        .unwrap();

    let full_path = format!("/gluster/{}", brick_name);
    let path = Path::new(&full_path);

    if path.exists() {
        println!("⚠️ El brick ya existe: {}", path.display());
        return;
    }

    match fs::create_dir_all(path) {
        Ok(_) => println!("✅ Brick creado: {}", path.display()),
        Err(e) => {
            eprintln!("❌ Error al crear el directorio: {e}");
            return;
        }
    }

    let user = whoami::username();

    let _ = Command::new("sudo")
        .arg("chown")
        .arg(format!("{user}:{user}"))
        .arg(&full_path)
        .status();

    let _ = Command::new("sudo")
        .arg("chmod")
        .arg("775")
        .arg(&full_path)
        .status();

    println!("🔐 Permisos y propiedad asignados correctamente.");
}

fn list_bricks() {
    let base_path = Path::new("/gluster/");
    println!("\n📄 Lista de bricks en /gluster/");

    match fs::read_dir(base_path) {
        Ok(entries) => {
            let mut count = 0;
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    println!("🧱 {}", entry.file_name().to_string_lossy());
                    count += 1;
                }
            }

            if count == 0 {
                println!("⚠️ No hay bricks creados.");
            }
        }
        Err(_) => println!("❌ No se pudo acceder a /gluster/. ¿Existe?"),
    }
}

fn delete_brick() {
    list_bricks();

    let brick_name: String = Input::new()
        .with_prompt("Nombre del brick que deseas eliminar")
        .interact_text()
        .unwrap();

    let full_path = format!("/gluster/{}", brick_name);
    let path = Path::new(&full_path);

    if !path.exists() {
        println!("❌ El brick no existe: {}", full_path);
        return;
    }

    if Confirm::new()
        .with_prompt(format!("¿Estás seguro de que quieres eliminar {}?", full_path))
        .default(false)
        .interact()
        .unwrap()
    {
        match fs::remove_dir_all(path) {
            Ok(_) => println!("🗑️ Brick eliminado correctamente."),
            Err(e) => println!("❌ No se pudo eliminar: {e}"),
        }
    }
}


pub fn manage_bricks() {
    loop {
        println!("\n🧱 Gestión de bricks GlusterFS");

        let options = vec!["Crear nuevo brick", "Listar bricks existentes", "Eliminar un brick", "Salir"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Elige una opción")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => create_brick(),
            1 => list_bricks(),
            2 => delete_brick(),
            3 => break,
            _ => continue,
        }
    }
}

