use std::fs;
use std::path::Path;
use std::process::Command;
use dialoguer::{Input, Select, Confirm};
use users::get_user_by_name;
use std::collections::HashSet;

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

            // 🔐 Solicitar nombre de usuario para asignar permisos
            let username: String = Input::new()
                .with_prompt("🔒 ¿A qué usuario quieres dar permisos del punto de montaje?")
                .interact_text()
                .unwrap();

            // Validar si el usuario existe en el sistema
            if get_user_by_name(&username).is_none() {
                println!("❌ El usuario '{}' no existe en el sistema.", username);
                return;
            }

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

fn is_protected_path(path: &str) -> bool {
    let protected = HashSet::from([
        "/", "/boot", "/home", "/etc", "/usr", "/var", "/bin", "/sbin", "/lib", "/lib64", "/mnt"
    ]);
    protected.contains(path)
}

pub fn manage_mounts() {
    println!("\n🧰 Gestión de puntos de montaje GlusterFS");

    // Obtener puntos de montaje desde `mount`
    let output = Command::new("mount")
        .output()
        .expect("❌ No se pudo ejecutar el comando `mount`");

    let mount_output = String::from_utf8_lossy(&output.stdout);
    let gluster_mounts: Vec<&str> = mount_output
        .lines()
        .filter(|line| line.contains("type glusterfs"))
        .collect();

    if gluster_mounts.is_empty() {
        println!("⚠️ No hay volúmenes GlusterFS montados.");
        return;
    }

    // Mostrar lista
    let items: Vec<String> = gluster_mounts
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                format!("{} (en {})", parts[0], parts[2])
            } else {
                line.to_string()
            }
        })
        .collect();

    let selection = Select::new()
        .with_prompt("Selecciona un volumen a desmontar")
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    let line = gluster_mounts[selection];
    let parts: Vec<&str> = line.split_whitespace().collect();

    if parts.len() < 3 {
        println!("❌ No se pudo interpretar el punto de montaje.");
        return;
    }

    let mount_path = parts[2];

    println!("🔽 Desmontando: {}", mount_path);
    let umount_status = Command::new("sudo")
        .arg("umount")
        .arg(mount_path)
        .status();

    match umount_status {
        Ok(status) if status.success() => {
            println!("✅ Desmontado correctamente.");

            if is_protected_path(mount_path) {
                println!("🛡️ Ruta protegida. No se puede eliminar.");
            } else {
                let remove = Confirm::new()
                    .with_prompt(format!("¿Deseas eliminar el directorio {}?", mount_path))
                    .default(false)
                    .interact()
                    .unwrap();

                if remove {
                    if let Err(e) = fs::remove_dir_all(Path::new(mount_path)) {
                        println!("⚠️ No se pudo eliminar: {}", e);
                    } else {
                        println!("🗑️ Directorio eliminado.");
                    }
                }
            }
        }
        _ => println!("❌ Falló el desmontaje."),
    }
}