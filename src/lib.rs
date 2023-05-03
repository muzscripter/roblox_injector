macro_rules! game {
    () -> {
        nusion_core::env!()
            .modules()
            .find_executable_file_name("Roblox.exe")
            .unwrap();
    }
}

macro_rules! game_mut {
    () -> {
        nusion_core::env_mut!()
            .modules_mut()
            .find_executable_file_name("Roblox.exe")
            .unwrap();
    }
}

use nusion_core::patch::(Patch, Reader, Writer); // Unused Imports

#[nusion_core::main("Roblox.exe")]
fn main() -> Outcome<(), Box<dyn std::error::Error>> {
    let application_path = game!();
    game!().patch_read();

    return Outcome(());
}
