use dialoguer::Select;

pub fn show_main_menu(is_master: bool) {
    loop {
        let mut options = vec![
            "Editar /etc/hosts",
            "Gestionar bricks",
            "Montar volumen",
            "Gestionar montajes", // ✅ Agregado aquí
            "Salir",
        ];

        if is_master {
            options.insert(2, "Agregar peer");
            options.insert(3, "Crear e iniciar volumen");
            options.insert(4, "Ver estado del clúster");
            options.insert(5, "Gestionar volúmenes");
        }

        let selection = Select::new()
            .with_prompt("¿Qué deseas hacer?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        if is_master {
            match selection {
                0 => crate::hosts::edit_hosts(),
                1 => crate::bricks::manage_bricks(),
                2 => crate::peers::add_peer(),
                3 => crate::volume::create_volume(),
                4 => crate::cluster::check_status(),
                5 => crate::volume::manage_volumes(),
                6 => crate::mount::mount_volume(),
                7 => crate::mount::manage_mounts(), // ✅ Acción correspondiente
                _ => break,
            }
        } else {
            match selection {
                0 => crate::hosts::edit_hosts(),
                1 => crate::bricks::manage_bricks(),
                2 => crate::mount::mount_volume(),
                3 => crate::mount::manage_mounts(), // ✅ Acción correspondiente
                _ => break,
            }
        }
    }
}
