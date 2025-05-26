use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use gtk::Application;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
use crate::{build_ui, desktop_entry, flags, user_details, AppState};

pub fn run_cli(is_global: bool, args: Vec<String>, local_share_applications: &str, global_share_applications: &str) -> std::io::Result<()> {

    let has_name = args.iter().any(|arg| arg == flags::NAME);
    let has_desktop_flags = args.iter().any(|arg|
        arg == flags::COMMENT ||
            arg == flags::EXEC_PATH ||
            arg == flags::ICON_PATH ||
            arg == flags::TERMINAL_APP ||
            arg == flags::APP_TYPE ||
            arg == flags::CATEGORIES
    );

    // If desktop flags are present but no --name, panic
    if has_desktop_flags && !has_name {
        panic!("Need to specify {} alongside passing details. Try again. Exiting.", flags::NAME);
    }

    // Get home directory
    let mut path = dirs::home_dir()
        .expect("Failed to get home directory");

    // Check if the user wants to install the desktop entry globally
    if is_global {
        // Check if running with sudo
        if !nix::unistd::getuid().is_root() {
            panic!("Global installation requires root privileges. Please run with sudo.");
        }
        path.push(global_share_applications);
    } else {
        path.push(local_share_applications);
    }

    // Create variables as containers for user input
    let mut name = String::new();
    let mut comment = String::new();
    let mut exec_path = String::new();
    let mut icon_path = String::new();
    let mut terminal_app = String::new();
    let mut app_type = String::new();
    let mut categories = String::new();



    let arg_name_value: Option<String> = args.iter()
        .position(|arg: &String| arg == flags::NAME)
        .and_then(|index| args.get(index + 1))
        .map(|value: &String| value.to_string());

    if let Some(_name) = &arg_name_value {
        // --name is provided, so .desktop details will be provided by flags / arguments
        // println!("Name provided via flag");
        name = _name.to_string();

        // println!("Name provided: {}", _name);

        let arg_comment_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::COMMENT)
            .and_then(|index| {
                // Collect all arguments after --comment until the next flag (starts with --)
                let mut comment_parts = Vec::new();
                let mut current_index = index + 1;

                while let Some(arg) = args.get(current_index) {
                    if arg.starts_with("--") {
                        break;
                    }
                    comment_parts.push(arg);
                    current_index += 1;
                }

                if comment_parts.is_empty() {
                    None
                } else {
                    Some(comment_parts.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(" "))
                }
            });

        if let Some(_comment) = &arg_comment_value {
            // println!("Comment provided: {}", _comment);
            comment = arg_comment_value.unwrap();
        }

        let arg_exec_path_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::EXEC_PATH)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_exec_path) = &arg_exec_path_value {
            // println!("Executable path provided: {}", _exec_path);
            exec_path = arg_exec_path_value.unwrap();
        }

        let arg_icon_path_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::ICON_PATH)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_icon_path) = &arg_icon_path_value {
            // println!("Icon path provided: {}", _icon_path);
            icon_path = arg_icon_path_value.unwrap();
        }

        let arg_terminal_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::TERMINAL_APP)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_terminal_app) = &arg_terminal_value {
            // println!("Terminal provided: {}", _terminal_app);
            terminal_app = arg_terminal_value.unwrap();
        }

        let arg_app_type_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::APP_TYPE)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_app_type) = &arg_app_type_value {
            // println!("App type provided: {}", _app_type);
            app_type = arg_app_type_value.unwrap();
        }

        let arg_categories_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::CATEGORIES)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_categories) = &arg_categories_value {
            // println!("Categories provided: {}", _categories);
            categories = arg_categories_value.unwrap();
        }


    } else {
        // --name has not been used, thus details will need to be provided by user through TUI

        // println!("Ask user for details");

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


    }

    // Create and write the desktop entry
    let filename = format!("{}.desktop", name.trim());
    path.push(filename);

    let mut file = File::create(&path)?;
    let entry = desktop_entry::DesktopEntry::new(
        name,
        comment,
        exec_path,
        icon_path,
        terminal_app,
        app_type,
        categories,
    );

    file.write_all(entry.to_string().as_bytes())?;
    println!("Desktop entry created at: {}", path.to_str().unwrap());

    Ok(())
}

pub fn run_gui(local_share_applications: &str) -> std::io::Result<()> {
    let state = Arc::new(Mutex::new(AppState::default()));

    let app = Application::builder()
        .application_id("com.catley.createdesktopfile")
        .build();

    let state_clone = Arc::clone(&state);
    app.connect_activate(move |app| build_ui(app, &state_clone));

    // Run the GUI application
    app.run();

    // After GUI closes, process the results
    let state_data = state.lock().unwrap();

    // Only proceed if the name is not empty (indicating the user submitted the form)
    if !state_data.name.is_empty() {
        let mut path = dirs::home_dir()
            .expect("Failed to get home directory");

        path.push(local_share_applications); // GUI mode always uses local installation
        path.push(format!("{}.desktop", state_data.name.trim()));

        let mut file = File::create(&path)?;
        let entry = desktop_entry::DesktopEntry::new(
            state_data.name.clone(),
            state_data.comment.clone(),
            state_data.exec_path.clone(),
            state_data.icon_path.clone(),
            state_data.terminal_app.clone(),
            state_data.app_type.clone(),
            state_data.categories.clone(),
        );

        file.write_all(entry.to_string().as_bytes())?;
        println!("Desktop entry created at: {}", path.to_str().unwrap());
    }

    Ok(())
}