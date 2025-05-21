use dialoguer::Input;
use std::process::Command;

pub fn mount_volume() {
    println!("\n📌 Montar volumen GlusterFS");

    let vol_name: String = Input::new()
        .with_prompt("Nombre del volumen a montar")
        .interact_text()
        .unwrap();

    let host: String = Input::new()
        .with_prompt("Hostname o IP del nodo Gluster que sirve el volumen")
        .interact_text()
        .unwrap();

    let mount_point: String = Input::new()
        .with_prompt("Directorio destino donde montar el volumen (debe existir)")
        .default("/mnt/gluster_vol".into())
        .interact_text()
        .unwrap();

    println!("🔧 Montando volumen...");

    let status = Command::new("sudo")
        .arg("mount")
        .arg("-t")
        .arg("glusterfs")
        .arg(format!("{}:{}", host, vol_name))
        .arg(&mount_point)
        .status()
        .expect("Error ejecutando mount");

    if status.success() {
        println!("✅ Volumen montado exitosamente en {}", mount_point);
    } else {
        println!("❌ Error al montar el volumen. Revisa que el directorio exista y que el volumen esté activo.");
    }
}
