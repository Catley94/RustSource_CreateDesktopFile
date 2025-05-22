mod desktop_entry;
mod user_details;
mod help_information;

use std::fs::File;
use std::io::Write;
use std::{env};

fn main() -> std::io::Result<()>{

    let global_arg = "--global";

    //Supported OSes
    let supported_oses: Vec<&str> = vec!["linux"];
    // Get current OS
    let os: &str = env::consts::OS;

    // Check if the OS is supported
    break_here_if_os_not_supported(supported_oses, &os);

    // Get all arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user wants to install the desktop entry globally
    let is_global = args.iter().any(|arg: &String| arg == global_arg);

    // Check if user wants to view help information
    if args.iter().any(|arg: &String | arg == "--help") {
        help_information::display_help_information(args);
        std::process::exit(0);
    }

    // Get home directory
    let mut path = dirs::home_dir()
        .expect("Failed to get home directory");

    // Define local and global paths to the where .desktop files should be moved to
    let local_share_applications_path = ".local/share/applications/";
    let global_share_applications_path = "/usr/share/applications/";

    // Check if the user wants to install the desktop entry globally
    if is_global {
        // Check if running with sudo
        if !nix::unistd::getuid().is_root() {
            panic!("Global installation requires root privileges. Please run with sudo.");
        }
        path.push(global_share_applications_path);
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

    // Ask user to populate details for .desktop file
    user_details::ask_user_to_fill_in_details(
        &mut name,
        &mut comment,
        &mut exec_path,
        &mut icon_path,
        &mut terminal_app,
        &mut app_type,
        &mut categories
    );

    // Create the name of the .desktop
    let filename = format!("{}.desktop", name.trim());

    // Add on the filename to the saved path location
    path.push(filename);

    // Create the .desktop file, currently empty
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

    // Write the desktop entry to the .desktop
    file.write_all(entry.to_string().as_bytes())?;

    println!("Desktop entry created at: {}", path.to_str().unwrap());

    // Print a success message
    Ok(())
}

fn break_here_if_os_not_supported(supported_oses: Vec<&str>, os: &&str) {
    if !supported_oses.contains(&os) {
        println!("--------------------------------------------------------------------");
        println!("This progam is only supported by the following Operating Systems:");
        println!("--------------------------------------------------------------------");
        supported_oses.iter().for_each(|os| println!("{}", os));
        println!("--------------------------------------------------------------------");
        panic!("This program is not running on a supported OS. Exiting.");
    }
}


