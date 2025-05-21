use dialoguer::{Input, Select, Confirm};
use std::process::Command;
use std::io::{self, Write};

pub fn create_volume() {
    println!("\n📦 Crear volumen GlusterFS");

    let vol_name: String = Input::new()
        .with_prompt("Nombre del volumen")
        .interact_text()
        .unwrap();

    let use_ha = Confirm::new()
        .with_prompt("¿Quieres habilitar alta disponibilidad (HA) con réplicas?")
        .default(true)
        .interact()
        .unwrap();

    // Comando base
    let mut cmd: Vec<String> = vec![
        "gluster".to_string(),
        "volume".to_string(),
        "create".to_string(),
        vol_name.clone(),
    ];

    let mut min_bricks = 1;

    if use_ha {
        let replica_count: String = Input::new()
            .with_prompt("¿Cuántas réplicas?")
            .default("2".into())
            .interact_text()
            .unwrap();

        cmd.push("replica".to_string());
        cmd.push(replica_count.clone());

        // Se requieren al menos tantas bricks como réplicas
        min_bricks = replica_count.parse::<usize>().unwrap_or(2);
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

    if bricks.len() < min_bricks {
        eprintln!(
            "❌ Se necesitan al menos {} bricks {}.",
            min_bricks,
            if use_ha { "para replicación (HA)" } else { "" }
        );
        return;
    }

    cmd.extend(bricks);
    cmd.push("force".to_string());

    println!("🚀 Ejecutando comando:");
    println!("sudo {}", cmd.join(" "));

    let status = Command::new("sudo")
        .arg("gluster")
        .args(&cmd[1..]) // todos menos el primero "gluster"
        .status()
        .expect("Error al ejecutar el comando");

    if status.success() {
        println!("✅ Volumen creado exitosamente.");
        let start_status = Command::new("sudo")
            .args(["gluster", "volume", "start", &vol_name])
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
            "📋 Listar volúmenes (con detalles)",
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
                // Mostrar información detallada del volumen
                let _ = Command::new("gluster")
                    .args(["volume", "info"])
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
                    .with_prompt("⚠️ ¿Estás seguro de que deseas eliminar este volumen?")
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

        // Esperar Enter antes de volver al menú
        print!("\nPresiona Enter para continuar...");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut String::new());
    }
}