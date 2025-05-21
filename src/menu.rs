use dialoguer::Select;

pub fn show_main_menu(is_master: bool) {
    loop {
        let mut options = vec![
            "Editar /etc/hosts",
            "Gestionar bricks",
            "Montar volumen",
            "Salir",
        ];

        if is_master {
            options.insert(2, "Agregar peer");
            options.insert(3, "Crear e iniciar volumen");
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
                4 => crate::mount::mount_volume(),
                _ => break,
            }
        } else {
            match selection {
                0 => crate::hosts::edit_hosts(),
                1 => crate::bricks::manage_bricks(),
                2 => crate::mount::mount_volume(),
                _ => break,
            }
        }
    }
}
