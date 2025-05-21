use std::process::Command;

pub fn check_status() {
    println!("Estado de los peers:");
    Command::new("gluster")
        .args(["peer", "status"])
        .status()
        .expect("Fallo al ejecutar gluster peer status");

    println!("\nInformación del volumen:");
    Command::new("gluster")
        .args(["volume", "info"])
        .status()
        .expect("Fallo al ejecutar gluster volume info");

    println!("\nEstado del volumen:");
    Command::new("gluster")
        .args(["volume", "status"])
        .status()
        .expect("Fallo al ejecutar gluster volume status");
}