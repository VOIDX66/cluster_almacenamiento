mod menu;
mod mode;
mod hosts;
mod bricks;
mod volume;
mod mount;
mod peers;
mod cluster;

fn main() {
    let is_master = mode::ask_role();
    menu::show_main_menu(is_master);
}
