use std::process::Command;

pub fn check_status() {
    println!("📡 Verificando estado del clúster...\n");

    println!("🔗 Estado de los peers:");
    let _ = Command::new("gluster")
        .args(["peer", "status"])
        .status()
        .expect("❌ Fallo al ejecutar 'gluster peer status'");

    println!("\n📦 Información del volumen:");
    let _ = Command::new("gluster")
        .args(["volume", "info"])
        .status()
        .expect("❌ Fallo al ejecutar 'gluster volume info'");

    println!("\n📈 Estado del volumen:");
    let _ = Command::new("gluster")
        .args(["volume", "status"])
        .status()
        .expect("❌ Fallo al ejecutar 'gluster volume status'");

    println!("\n✅ Consulta completada.\n");
}
