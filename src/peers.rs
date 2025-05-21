use dialoguer::Input;
use std::process::Command;

pub fn add_peer() {
    println!("\n➕ Añadir nuevo peer (nodo) al cluster");

    let peer_host: String = Input::new()
        .with_prompt("Hostname o IP del nodo a añadir")
        .interact_text()
        .unwrap();

    println!("🔧 Ejecutando: sudo gluster peer probe {}", peer_host);

    let status = Command::new("sudo")
        .arg("gluster")
        .arg("peer")
        .arg("probe")
        .arg(&peer_host)
        .status()
        .expect("Error ejecutando gluster peer probe");

    if status.success() {
        println!("✅ Nodo '{}' añadido correctamente al cluster.", peer_host);
    } else {
        println!("❌ No se pudo añadir el nodo '{}'. Revisa la conexión y que el nodo esté disponible.", peer_host);
    }
}