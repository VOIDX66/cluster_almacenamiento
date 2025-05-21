use std::fs;
use std::path::Path;
use std::process::Command;
use dialoguer::Input;

pub fn mount_volume() {
    println!("\n📂 Montar volumen GlusterFS");

    let server: String = Input::new()
        .with_prompt("Nombre del servidor (ej. vm1)")
        .interact_text()
        .unwrap();

    let volume: String = Input::new()
        .with_prompt("Nombre del volumen")
        .interact_text()
        .unwrap();

    let mount_point: String = Input::new()
        .with_prompt("Ruta donde montar (ej. /mnt/vol_personal)")
        .interact_text()
        .unwrap();

    // ✅ Crear el directorio si no existe
    let path = Path::new(&mount_point);
    if !path.exists() {
        println!("📁 La ruta no existe. Creando...");
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("❌ No se pudo crear la ruta de montaje: {}", e);
            return;
        }
    }

    println!("🚀 Ejecutando comando:");
    println!("sudo mount -t glusterfs {}:/{} {}", server, volume, mount_point);

    let status = Command::new("sudo")
        .arg("mount")
        .arg("-t")
        .arg("glusterfs")
        .arg(format!("{}:/{}", server, volume))
        .arg(&mount_point)
        .status();

    if let Ok(s) = status {
        if s.success() {
            println!("✅ Volumen montado exitosamente.");
            use whoami;

            let username = whoami::username();

            let chown_status = Command::new("sudo")
                .arg("chown")
                .arg(format!("{}:{}", username, username))
                .arg(&mount_point)
                .status();

            if let Ok(cs) = chown_status {
                if cs.success() {
                    println!("✅ Permisos cambiados a usuario: {}", username);
                } else {
                    println!("⚠️ No se pudo cambiar la propiedad del punto de montaje.");
                }
            }
            
        } else {
            println!("❌ Falló el montaje. Verifica que el volumen esté iniciado y que tengas permisos.");
        }
    } else {
        println!("❌ Error al ejecutar el comando de montaje.");
    }
}
