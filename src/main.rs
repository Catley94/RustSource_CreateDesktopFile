mod desktop_entry;
mod user_details;

use std::fs::File;
use std::io::Write;
use std::{env};

fn main() -> std::io::Result<()>{

    let global_arg = "--global";
    let supported_oses: Vec<&str> = vec!["linux"];

    // Check if this is a Linux OS, if it isn't, throw an error
    let os: &str = env::consts::OS;

    if !supported_oses.contains(&os) {
        println!("--------------------------------------------------------------------");
        println!("This progam is only supported by the following Operating Systems:");
        println!("--------------------------------------------------------------------");
        supported_oses.iter().for_each(|os| println!("{}", os));
        println!("--------------------------------------------------------------------");
        panic!("This program is not running on a supported OS. Exiting.");
    }

    let args: Vec<String> = env::args().collect();
    let is_global = args.iter().any(|arg: &String| arg == global_arg);
    if args.iter().any(|arg| arg == "--help") {
        println!("Usage: {} [--global]", args[0]);
        println!("Options:");
        println!("  --local     (Default) Install .desktop file locally in .local/share/applications/");
        println!("  --global    Install .desktop file globally in /usr/share/applications/");
        println!("  --help      Show this help message");
        std::process::exit(0);
    }


    // Path to the where the .desktop file should be moved to
    let mut path = dirs::home_dir().expect("Failed to get home directory");
    let local_share_applications_path = ".local/share/applications/";
    let global_share_applications_path = "/usr/share/applications/";
    if is_global {
        // Check if running with sudo
        if !nix::unistd::getuid().is_root() {
            panic!("Global installation requires root privileges. Please run with sudo.");
        }
        path = global_share_applications_path.into();
    } else {
        path.push(local_share_applications_path);
    }


    // Create variables as containers for user input
    let mut name: String = String::new();
    let mut comment: String = String::new();
    let mut exec_path: String = String::new();
    let mut icon_path: String = String::new();
    let mut terminal_app: String = String::new();
    let mut app_type: String = String::new();
    let mut categories: String = String::new();

    // Ask user to populate details
    user_details::ask_user_to_fill_in_details(
        &mut name,
        &mut comment,
        &mut exec_path,
        &mut icon_path,
        &mut terminal_app,
        &mut app_type,
        &mut categories
    );

    // Create the desktop entry file
    let filename = format!("{}.desktop", name.trim());

    // Add on the filename to the saved path location
    path.push(filename);
    let mut file = File::create(&path)?;

    // Create the desktop entry
    let entry = desktop_entry::DesktopEntry::new(
        name,
        comment,
        exec_path,
        icon_path,
        terminal_app,
        app_type,
        categories,
    );

    // Write the desktop entry to the file
    file.write_all(entry.to_string().as_bytes())?;

    println!("Desktop entry created at: {}", path.to_str().unwrap());

    // Print a success message
    Ok(())
}

