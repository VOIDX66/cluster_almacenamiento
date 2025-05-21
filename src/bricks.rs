use dialoguer::{Input, Confirm};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn manage_bricks() {
    println!("\n🧱 Configuración del directorio brick");

    let default_path = "/gluster/brick1";
    let path: String = Input::new()
        .with_prompt("Ruta del brick")
        .default(default_path.to_string())
        .interact_text()
        .unwrap();

    let path = Path::new(&path);

    if path.exists() {
        println!("✅ El directorio ya existe: {}", path.display());
    } else {
        match fs::create_dir_all(path) {
            Ok(_) => println!("✅ Directorio creado: {}", path.display()),
            Err(e) => {
                eprintln!("❌ Error al crear el directorio: {e}");
                return;
            }
        }
    }

    // Asignar permisos y propiedad al usuario actual
    let user = whoami::username();

    println!("🔧 Asignando permisos al usuario '{}'", user);

    // Nota: asumimos que el grupo es igual al usuario
    let chown_status = Command::new("sudo")
        .arg("chown")
        .arg(format!("{user}:{user}"))
        .arg(path)
        .status();

    if let Ok(status) = chown_status {
        if status.success() {
            println!("✅ Propiedad asignada correctamente.");
        } else {
            eprintln!("❌ No se pudo cambiar la propiedad. Usa sudo manualmente si es necesario.");
        }
    }

    let chmod_status = Command::new("sudo")
        .arg("chmod")
        .arg("775")
        .arg(path)
        .status();

    if let Ok(status) = chmod_status {
        if status.success() {
            println!("✅ Permisos establecidos a 775.");
        }
    }

    if Confirm::new()
        .with_prompt("¿Quieres crear otro directorio brick?")
        .interact()
        .unwrap()
    {
        manage_bricks();
    }
}
