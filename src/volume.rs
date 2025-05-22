use dialoguer::{Input, Select, Confirm, theme::ColorfulTheme};
use std::process::Command;
use std::io::{self, Write};
use std::str;

fn run_command(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| format!("Error al ejecutar comando: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

fn list_volumes() -> Vec<String> {
    match run_command("gluster", &["volume", "info"]) {
        Ok(output) => {
            // Extraemos nombres de volumen, buscando líneas que empiecen con "Volume Name: "
            output.lines()
                .filter_map(|line| {
                    if let Some(name) = line.strip_prefix("Volume Name: ") {
                        Some(name.trim().to_string())
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(e) => {
            eprintln!("⚠️ Error listando volúmenes: {}", e);
            vec![]
        }
    }
}

fn list_peers() -> Vec<String> {
    match run_command("gluster", &["peer", "status"]) {
        Ok(output) => {
            // Buscamos líneas que contienen "Hostname: <host>"
            output.lines()
                .filter_map(|line| {
                    if let Some(host) = line.trim().strip_prefix("Hostname: ") {
                        Some(host.to_string())
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(e) => {
            eprintln!("⚠️ Error listando peers: {}", e);
            vec![]
        }
    }
}

fn list_bricks(volume: &str) -> Vec<String> {
    match run_command("gluster", &["volume", "info", volume]) {
        Ok(output) => {
            // Extraemos líneas con bricks: "Brick1: vm1:/ruta/brick"
            output.lines()
                .filter_map(|line| {
                    if let Some(brick) = line.trim().strip_prefix("Brick") {
                        // brick tiene formato "1: vm1:/ruta/brick"
                        if let Some(pos) = brick.find(':') {
                            Some(brick[(pos+1)..].trim().to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect()
        }
        Err(e) => {
            eprintln!("⚠️ Error listando bricks: {}", e);
            vec![]
        }
    }
}

pub fn create_volume() {
    println!("\n📦 Crear volumen GlusterFS");

    let theme = ColorfulTheme::default();

    let vol_name: String = Input::with_theme(&theme)
        .with_prompt("Nombre del volumen (o escribe 'salir' para cancelar)")
        .interact_text()
        .unwrap();

    if vol_name.trim().eq_ignore_ascii_case("salir") {
        println!("❎ Operación cancelada.");
        return;
    }

    println!("🧱 Ahora ingresa los bricks para este volumen.");
    println!("Formato: vm1:/ruta/brick1 (uno por línea). Escribe 'fin' para terminar o 'salir' para cancelar completamente.");

    let mut bricks: Vec<String> = Vec::new();

    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Brick")
            .interact_text()
            .unwrap();

        let trimmed = input.trim().to_lowercase();

        if trimmed == "salir" {
            println!("❎ Operación cancelada.");
            return;
        }

        if trimmed == "fin" {
            break;
        }

        if input.contains(':') && input.contains('/') {
            bricks.push(input);
        } else {
            println!("⚠️ Formato inválido. Usa: vm1:/ruta/brick");
        }
    }

    if bricks.is_empty() {
        eprintln!("❌ Se necesita al menos 1 brick para crear el volumen.");
        return;
    }

    let mut cmd: Vec<String> = vec![
        "gluster".to_string(),
        "volume".to_string(),
        "create".to_string(),
        vol_name.clone(),
    ];
    cmd.extend(bricks.iter().cloned());
    cmd.push("force".to_string());

    println!("🚀 Ejecutando comando:");
    println!("sudo {}", cmd.join(" "));

    let status = Command::new("sudo")
        .arg("gluster")
        .args(&cmd[1..])
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

pub fn add_bricks() {
    let theme = ColorfulTheme::default();

    let mut volumes = list_volumes();
    if volumes.is_empty() {
        println!("⚠️ No hay volúmenes para agregar bricks.");
        return;
    }

    volumes.insert(0, "Salir".to_string());

    let vol_idx = Select::with_theme(&theme)
        .with_prompt("Selecciona el volumen al que quieres agregar bricks")
        .items(&volumes)
        .default(0)
        .interact()
        .unwrap();

    if vol_idx == 0 {
        println!("❎ Operación cancelada.");
        return;
    }

    let selected_vol = &volumes[vol_idx];

    let peers = list_peers();
    if peers.is_empty() {
        println!("⚠️ No hay peers conectados. No puedes agregar bricks.");
        return;
    }

    println!("Peers disponibles:");
    for p in &peers {
        println!("- {}", p);
    }

    println!("🧱 Ingresa los bricks para agregar al volumen.");
    println!("Formato: vm1:/ruta/brick1 (uno por línea). Escribe 'fin' para terminar o 'salir' para cancelar toda la operación.");

    let mut bricks_to_add: Vec<String> = Vec::new();

    loop {
        let input: String = Input::with_theme(&theme)
            .with_prompt("Brick")
            .interact_text()
            .unwrap();

        let trimmed = input.trim().to_lowercase();

        if trimmed == "salir" {
            if bricks_to_add.is_empty() {
                println!("❎ Operación cancelada.");
                return;
            } else {
                let confirm = dialoguer::Confirm::with_theme(&theme)
                    .with_prompt("Ya has ingresado bricks válidos. ¿Seguro que quieres salir sin aplicar los cambios?")
                    .default(false)
                    .interact()
                    .unwrap();
                if confirm {
                    println!("❎ Operación cancelada.");
                    return;
                } else {
                    continue;
                }
            }
        }

        if trimmed == "fin" {
            break;
        }

        if input.contains(':') && input.contains('/') {
            let host = input.split(':').next().unwrap_or("");
            if peers.contains(&host.to_string()) {
                bricks_to_add.push(input);
            } else {
                println!("⚠️ El host '{}' no está entre los peers conectados.", host);
            }
        } else {
            println!("⚠️ Formato inválido. Usa: vm1:/ruta/brick");
        }
    }

    if bricks_to_add.is_empty() {
        println!("❌ No agregaste bricks válidos.");
        return;
    }

    let mut cmd = vec![
        "gluster".to_string(),
        "volume".to_string(),
        "add-brick".to_string(),
        selected_vol.clone(),
    ];
    cmd.extend(bricks_to_add);
    cmd.push("force".to_string());

    println!("🚀 Ejecutando comando:");
    println!("sudo {}", cmd.join(" "));

    let status = Command::new("sudo")
        .arg("gluster")
        .args(&cmd[1..])
        .status()
        .expect("Error al ejecutar el comando");

    if status.success() {
        println!("✅ Bricks agregados exitosamente.");
    } else {
        println!("❌ Error al agregar bricks.");
    }
}

fn check_force_migration(volume: &str) -> Result<bool, String> {
    let args = ["volume", "get", volume, "cluster.force-migration"];
    match run_command("gluster", &args) {
        Ok(output) => {
            for line in output.lines() {
                if line.contains("cluster.force-migration") {
                    if line.contains("on") {
                        return Ok(true);
                    } else if line.contains("off") {
                        return Ok(false);
                    }
                }
            }
            Err("No se pudo determinar el estado de cluster.force-migration.".into())
        }
        Err(e) => Err(format!("Error al obtener cluster.force-migration: {}", e)),
    }
}

pub fn remove_bricks() {
    let theme = ColorfulTheme::default();

    let mut volumes = list_volumes();
    if volumes.is_empty() {
        println!("⚠️ No hay volúmenes para eliminar bricks.");
        return;
    }

    volumes.insert(0, "Salir".to_string());

    let vol_idx = Select::with_theme(&theme)
        .with_prompt("Selecciona el volumen del que quieres eliminar bricks")
        .items(&volumes)
        .default(0)
        .interact()
        .unwrap();

    if vol_idx == 0 {
        println!("❎ Operación cancelada.");
        return;
    }

    let selected_vol = &volumes[vol_idx];

    // ⚠️ Verificación de configuración peligrosa
    match check_force_migration(selected_vol) {
        Ok(true) => {
            println!("⚠️ Advertencia: cluster.force-migration está habilitado (ON) en el volumen '{}'. Esto puede causar corrupción de datos al eliminar bricks.", selected_vol);
            if !Confirm::with_theme(&theme)
                .with_prompt("¿Quieres continuar con la eliminación del brick igual? (no recomendado)")
                .default(false)
                .interact()
                .unwrap()
            {
                println!("🛑 Operación cancelada por seguridad.");
                return;
            }
        }
        Ok(false) => {} // nada
        Err(e) => {
            println!("⚠️ No se pudo verificar cluster.force-migration: {}", e);
            println!("Continuando con precaución...");
        }
    }

    let mut bricks = list_bricks(selected_vol);
    if bricks.is_empty() {
        println!("⚠️ Este volumen no tiene bricks listados o no se pudieron obtener.");
        return;
    }

    bricks.insert(0, "❌ Salir".to_string());

    let brick_idx = Select::with_theme(&theme)
        .with_prompt("Selecciona el brick que quieres eliminar")
        .items(&bricks)
        .default(0)
        .interact()
        .unwrap();

    if brick_idx == 0 {
        println!("❎ Operación cancelada.");
        return;
    }

    let selected_brick = &bricks[brick_idx];

    if !Confirm::with_theme(&theme)
        .with_prompt(format!(
            "⚠️ ¿Seguro que deseas eliminar el brick '{}' del volumen '{}'? Esto puede afectar los datos.",
            selected_brick, selected_vol
        ))
        .default(false)
        .interact()
        .unwrap()
    {
        println!("🛑 Operación cancelada.");
        return;
    }

    println!("🚀 Iniciando eliminación del brick...");
    let start_status = Command::new("sudo")
        .args(&[
            "gluster",
            "volume",
            "remove-brick",
            selected_vol,
            selected_brick,
            "start",
        ])
        .status();

    match start_status {
        Ok(st) if st.success() => {
            println!("✅ Proceso de eliminación iniciado.");
            println!("ℹ️ Recuerda ejecutar el comando de confirmación:");
            println!("   sudo gluster volume remove-brick {} {} commit", selected_vol, selected_brick);
        }
        Ok(_) => {
            println!("❌ Falló iniciar la eliminación del brick.");
        }
        Err(e) => {
            println!("❌ Error al ejecutar el comando: {}", e);
        }
    }
}

fn get_volume_names() -> Vec<String> {
    let output = match Command::new("gluster").args(["volume", "list"]).output() {
        Ok(out) => out,
        Err(_) => return vec![], // Devuelve lista vacía si falla
    };

    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn manage_volumes() {
    let theme = ColorfulTheme::default();

    loop {
        let options = vec![
            "📋 Listar volúmenes (con detalles)",
            "▶️ Iniciar volumen",
            "⏹️ Detener volumen",
            "🗑️ Eliminar volumen",
            "➕ Agregar bricks a volumen",
            "➖ Eliminar bricks de volumen",
            "↩️ Volver al menú",
        ];

        let selection = Select::with_theme(&theme)
            .with_prompt("Gestión de volúmenes")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                match Command::new("gluster").args(["volume", "info"]).output() {
                    Ok(output) => println!("{}", String::from_utf8_lossy(&output.stdout)),
                    Err(e) => eprintln!("⚠️ Error listando volúmenes: {}", e),
                }
            }
            1 | 2 | 3 => {
                let mut volumes = get_volume_names();
                if volumes.is_empty() {
                    println!("⚠️ No hay volúmenes disponibles.");
                    continue;
                }

                // Insertamos la opción salir primero
                volumes.insert(0, "Salir".to_string());

                let vol_index = Select::with_theme(&theme)
                    .with_prompt("Selecciona el volumen")
                    .items(&volumes)
                    .default(0)
                    .interact()
                    .unwrap();

                if vol_index == 0 {
                    println!("❎ Operación cancelada.");
                    continue; // Regresa al menú principal
                }

                let name = &volumes[vol_index];

                match selection {
                    1 => {
                        match Command::new("sudo")
                            .args(["gluster", "volume", "start", name])
                            .status()
                        {
                            Ok(st) if st.success() => println!("✅ Volumen iniciado."),
                            _ => println!("❌ Falló iniciar volumen."),
                        }
                    }
                    2 => {
                        match Command::new("sudo")
                            .args(["gluster", "volume", "stop", name, "force"])
                            .status()
                        {
                            Ok(st) if st.success() => println!("✅ Volumen detenido."),
                            _ => println!("❌ Falló detener volumen."),
                        }
                    }
                    3 => {
                        if Confirm::with_theme(&theme)
                            .with_prompt(format!("⚠️ ¿Eliminar el volumen '{}'?", name))
                            .default(false)
                            .interact()
                            .unwrap()
                        {
                            match Command::new("sudo")
                                .args(["gluster", "volume", "delete", name])
                                .status()
                            {
                                Ok(st) if st.success() => println!("✅ Volumen eliminado."),
                                _ => println!("❌ Falló eliminar volumen."),
                            }
                        } else {
                            println!("🛑 Eliminación cancelada.");
                        }
                    }
                    _ => {}
                }
            }
            4 => add_bricks(),
            5 => remove_bricks(),
            _ => break,
        }

        print!("\nPresiona Enter para continuar...");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(&mut String::new());
    }
}