use dll_mod_loader::{ ModLoader, ModLoaderError };

fn main() {
    if let Err(err) = run_mod_loader() {
        eprintln!("Error: {}", err);
        println!("Exiting...");
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}

fn run_mod_loader() -> Result<(), ModLoaderError> {
    let mut loader = ModLoader::new();

    loader.create_suspended_process(".\\supertux2.exe", Some(3014656))?;
    loader.load_mods(".\\Mods")?;
    loader.inject_mods()?;
    loader.resume_process()?;

    println!("Exiting...");
    std::thread::sleep(std::time::Duration::from_secs(3));

    Ok(())
}
