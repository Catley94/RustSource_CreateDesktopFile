//! Contains modules and components required for desktop entry generation.
mod desktop_entry;
mod user_details;
mod flags;
mod help_information;
mod path;
mod desktop_entry_tests;
mod modes;

use std::fs::File;
use std::io::Write;
use std::{env};
use gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Grid, Label};
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct AppState {
    name: String,
    comment: String,
    exec_path: String,
    icon_path: String,
    terminal_app: String,
    app_type: String,
    categories: String,
}

fn main() -> std::io::Result<()> {

    // Flags supported by the application

    //Supported OSes
    let supported_oses: Vec<&str> = vec!["linux"];
    let os: &str = env::consts::OS;
    break_here_if_os_not_supported(supported_oses, &os);

    // Get all arguments
    let args: Vec<String> = env::args().collect();
    
    // Check for CLI flags
    let is_cli = args.iter().any(|arg| arg == flags::LOCAL || arg == flags::GLOBAL || arg == flags::NAME);
    let is_global = args.iter().any(|arg| arg == flags::GLOBAL);


    // Check if user wants to view help information first
    if args.iter().any(|arg| arg == flags::HELP) {
        help_information::display_help_information(args);
        std::process::exit(0);
    }
    
    // Check if user wants to view version
    if args.iter().any(|arg: &String| arg == flags::VERSION) {
        println!("CreateDesktopFile v{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    
    if is_cli {
        // Run CLI version
        modes::run_cli(is_global, args, path::LOCAL_SHARE_APPLICATIONS, path::GLOBAL_SHARE_APPLICATIONS)?;
    } else {
        // Run GUI version
        modes::run_gui(path::LOCAL_SHARE_APPLICATIONS)?;
    }

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

fn build_ui(app: &Application, state: &Arc<Mutex<AppState>>) {
    let grid = Grid::builder()
        .margin_start(12)
        .margin_end(12)
        .margin_top(12)
        .margin_bottom(12)
        .row_spacing(6)
        .column_spacing(12)
        .build();

    // Create labels and entries with their corresponding field names
    let entries = [
        ("name", Label::new(Some("Name:")), Entry::new()),
        ("comment", Label::new(Some("Comment:")), Entry::new()),
        ("exec_path", Label::new(Some("Executable Path:")), Entry::new()),
        ("icon_path", Label::new(Some("Icon Path:")), Entry::new()),
        ("terminal_app", Label::new(Some("Terminal App?:")), Entry::new()),
        ("categories", Label::new(Some("Categories:")), Entry::new()),
    ];


    // Add labels and entries to the grid
    for (i, (_, label, entry)) in entries.iter().enumerate() {
        label.set_halign(gtk::Align::End);
        grid.attach(label, 0, i as i32, 1, 1);
        grid.attach(entry, 1, i as i32, 1, 1);
        entry.set_hexpand(true);
    }

    let button = Button::with_label("Generate");
    grid.attach(&button, 0, 6, 2, 1);
    button.set_margin_top(12);
    button.set_hexpand(true);

    // Add the informational label below the button
    let info_label = Label::new(Some(format!("This will only create .desktop files within ~/{}", path::LOCAL_SHARE_APPLICATIONS).as_str()));
    info_label.set_margin_top(12);
    info_label.set_wrap(true);
    info_label.set_margin_start(6);
    grid.attach(&info_label, 0, 7, 2, 1);  // Attach to row 7 (after the button which is at row 6)

    // Create the window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Desktop Entry Generator")
        .child(&grid)
        .default_width(400)
        .build();

    // Create clones for the closure
    let entries_clone = entries.to_vec();
    let state_clone = Arc::clone(&state);
    let window_clone = window.clone();

    // Handle button click
    button.connect_clicked(move |_| {
        let mut state = state_clone.lock().unwrap();

        // Update state with values from entries
        for (field_name, _, entry) in &entries_clone {
            let value = entry.text().to_string();
            match *field_name {
                "name" => state.name = value,
                "comment" => state.comment = value,
                "exec_path" => state.exec_path = value,
                "icon_path" => state.icon_path = value,
                "terminal_app" => state.terminal_app = value,
                "categories" => state.categories = value,
                _ => {}
            }
        }

        // Create and save the desktop entry
        if !state.name.is_empty() {
            let mut path = dirs::home_dir()
                .expect("Failed to get home directory");

            path.push(path::LOCAL_SHARE_APPLICATIONS);
            path.push(format!("{}.desktop", state.name.trim()));

            // Create the desktop entry
            let entry = desktop_entry::DesktopEntry::new(
                state.name.clone(),
                state.comment.clone(),
                state.exec_path.clone(),
                state.icon_path.clone(),
                state.terminal_app.clone(),
                String::from("Application"), // Default app_type
                state.categories.clone(),
            );

            // Create directory if it doesn't exist
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                    eprintln!("Failed to create directory: {}", e);
                });
            }

            // Write the file
            if let Ok(mut file) = File::create(&path) {
                if let Ok(_) = file.write_all(entry.to_string().as_bytes()) {
                    // Show success message
                    let dialog = gtk::MessageDialog::new(
                        Some(&window_clone),
                        gtk::DialogFlags::MODAL,
                        gtk::MessageType::Info,
                        gtk::ButtonsType::Ok,
                        &format!("Successfully created .desktop file at:\n{}",
                                 path.to_str().unwrap_or(""))
                    );

                    dialog.connect_response(|dialog, _| {
                        dialog.close();
                    });

                    dialog.show();

                    // Clear all entry fields
                    for (_, _, entry) in &entries_clone {
                        entry.set_text("");
                    }
                }
            }
        }
    });

    window.present();

}