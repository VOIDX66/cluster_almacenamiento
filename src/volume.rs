use dialoguer::{Input, Select, Confirm};
use std::process::Command;

pub fn create_volume() {
    println!("\n📦 Crear volumen GlusterFS");

    let vol_name: String = Input::new()
        .with_prompt("Nombre del volumen")
        .interact_text()
        .unwrap();

    let is_replica = Confirm::new()
        .with_prompt("¿Es un volumen replicado?")
        .default(true)
        .interact()
        .unwrap();

    // cmd es Vec<String>
    let mut cmd: Vec<String> = vec![
        "gluster".to_string(),
        "volume".to_string(),
        "create".to_string(),
        vol_name.clone(),
    ];

    if is_replica {
        let replica_count: String = Input::new()
            .with_prompt("¿Cuántas réplicas?")
            .default("2".into())
            .interact_text()
            .unwrap();

        cmd.push("replica".to_string());
        cmd.push(replica_count);
    }

    println!("🧱 Ahora ingresa los bricks para este volumen.");
    println!("Formato: vm1:/ruta/brick1 (uno por línea, escribe 'fin' para terminar)");

    let mut bricks: Vec<String> = Vec::new();

    loop {
        let input: String = Input::new()
            .with_prompt("Brick")
            .interact_text()
            .unwrap();

        if input.trim().to_lowercase() == "fin" {
            break;
        }

        if input.contains(':') && input.contains('/') {
            bricks.push(input);
        } else {
            println!("⚠️ Formato inválido. Usa: vm1:/ruta/brick");
        }
    }

    if bricks.len() < 2 && is_replica {
        eprintln!("❌ Se necesitan al menos 2 bricks para replicación.");
        return;
    }

    for brick in bricks {
        cmd.push(brick);
    }

    cmd.push("force".to_string());

    println!("🚀 Ejecutando comando:");
    println!("sudo {}", cmd.join(" "));

    let status = Command::new("sudo")
        .arg("gluster")
        .args(&cmd[1..]) // los argumentos después de "gluster"
        .status()
        .expect("Error al ejecutar el comando");

    if status.success() {
        println!("✅ Volumen creado exitosamente.");
        // Intentamos iniciar el volumen automáticamente
        let start_status = Command::new("sudo")
            .arg("gluster")
            .arg("volume")
            .arg("start")
            .arg(&vol_name)
            .status();

        if let Ok(st) = start_status {
            if st.success() {
                println!("✅ Volumen iniciado correctamente.");
            } else {
                println!("⚠️ Volumen creado, pero no pudo iniciarse.");
            }
        }
    } else {
        println!("❌ Error al crear el volumen.");
    }
}

pub fn manage_volumes() {
    loop {
        let options = vec![
            "📋 Listar volúmenes",
            "▶️ Iniciar volumen",
            "⏹️ Detener volumen",
            "🗑️ Eliminar volumen",
            "↩️ Volver al menú",
        ];

        let selection = Select::new()
            .with_prompt("Gestión de volúmenes")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                let _ = Command::new("gluster")
                    .args(["volume", "list"])
                    .status();
            }
            1 => {
                let name: String = Input::new()
                    .with_prompt("Nombre del volumen a iniciar")
                    .interact_text()
                    .unwrap();
                let _ = Command::new("sudo")
                    .args(["gluster", "volume", "start", &name])
                    .status();
            }
            2 => {
                let name: String = Input::new()
                    .with_prompt("Nombre del volumen a detener")
                    .interact_text()
                    .unwrap();
                let _ = Command::new("sudo")
                    .args(["gluster", "volume", "stop", &name, "force"])
                    .status();
            }
            3 => {
                let name: String = Input::new()
                    .with_prompt("Nombre del volumen a eliminar")
                    .interact_text()
                    .unwrap();

                if Confirm::new()
                    .with_prompt("¿Estás seguro de que deseas eliminar este volumen?")
                    .default(false)
                    .interact()
                    .unwrap()
                {
                    let _ = Command::new("sudo")
                        .args(["gluster", "volume", "delete", &name])
                        .status();
                }
            }
            4 => break,
            _ => break,
        }
    }
}