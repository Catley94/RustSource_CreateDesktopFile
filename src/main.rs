//! Contains modules and components required for desktop entry generation.
mod desktop_entry;
mod user_details;
mod flags;
mod help_information;

use std::fs::File;
use std::io::Write;
use std::{env};
use gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Entry, Grid, Label};
use std::sync::{Arc, Mutex};


/// The `AppState` struct is used to represent and manage the state of an application
/// with associated metadata fields.
///
/// # Fields
/// - `name` (`String`): The name of the application.
/// - `comment` (`String`): A brief description or comment about the application.
/// - `exec_path` (`String`): The file path to the executable for the application.
/// - `icon_path` (`String`): The file path to the application's icon.
/// - `terminal_app` (`String`): Specifies whether the application requires a terminal to run.
///   Typically holds values like `"true"` or `"false"`.
/// - `app_type` (`String`): The type of the application, e.g., "Utility", "Game", etc.
/// - `categories` (`String`): A comma-separated list of categories to which the application belongs.
///
/// # Default Implementation
/// By deriving `Default`, an instance of `AppState` can be created with all fields set
/// to their default value, which is an empty `String` for each field.
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


/// The entry point of the application. Determines the mode of operation (CLI or GUI)
/// based on the provided command-line arguments and executes the corresponding logic.
///
/// # Behavior
/// - Parses command-line arguments to identify if the application should run in CLI or GUI mode.
/// - Offers a `--help` flag to display usage instructions and exits immediately if provided.
/// - Recognizes the following CLI mode flags:
///   - `--cli`: Runs the application in CLI mode.
///   - `--local`: Sets up for local execution (used in CLI mode).
///   - `--global`: Sets up for global execution (used in CLI mode).
/// - If no CLI-specific flags are detected, the application defaults to GUI mode.
///
/// # Returns
/// - `std::io::Result<()>`: Returns `Ok(())` on successful execution or an error if something goes wrong during execution.
///
/// # Usage
/// ```sh
/// # Run the application in GUI mode (default)
/// ./CreateDesktopFile
///
/// # Run the application in CLI mode (local)
/// ./CreateDesktopFile --cli --local
///
/// # Run the application in CLI mode (global)
/// ./CreateDesktopFile --cli --global
///
/// # Display help information
/// ./CreateDesktopFile --help
/// ```
///
/// # Error Handling
/// - Handles errors from the `run_cli_mode` and `run_gui_mode` functions and propagates them back to the caller.
///
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


    let has_name = args.iter().any(|arg| arg == "--name");
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
        panic!("Need to specify --name alongside passing details. Try again. Exiting.");
    }


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
        run_cli_mode(is_global, args)?;
    } else {
        // Run GUI version
        run_gui_mode()?;
    }

    Ok(())
}

/// Executes the CLI mode for creating a `.desktop` entry file.
///
/// This function provides an interactive command-line interface that allows
/// users to populate details for creating a `.desktop` file, which is used
/// to define application shortcuts in Linux environments. Depending on whether
/// the operation is global or local, the `.desktop` file is saved in the
/// appropriate directory.
///
/// # Arguments
///
/// * `is_global` - A boolean flag indicating whether the desktop entry should
///   be installed globally (`true`) or locally (`false`).
///
/// # Returns
///
/// Returns a `std::io::Result<()>` indicating the success or failure of the
/// operation. If the operation is successful, it returns `Ok(())`; otherwise,
/// an error is returned.
///
/// # Behavior
///
/// 1. **OS Support Check**:
///    - The function validates whether the current OS is supported. Currently,
///      only Linux is supported. An error occurs if the function is called on
///      an unsupported OS.
///
/// 2. **Directory Setup**:
///    - Retrieves the user's home directory to construct the target path for the
///      `.desktop` file.
///    - Depending on the `is_global` flag:
///      - If `true` (global installation), the user must have root privileges
///        (checked via `nix::unistd::getuid`). The `.desktop` file is placed
///        in `/usr/share/applications/`.
///      - If `false` (local installation), the file is saved in
///        `$HOME/.local/share/applications/`.
///
/// 3. **User Input**:
///    - Prompts the user to provide details for the `.desktop` file such as:
///      - `Name`: Name of the application.
///      - `Comment`: Description of the application.
///      - `Exec`: Path to the executable.
///      - `Icon`: Path to the icon file.
///      - `Terminal`: Whether the application should run in a terminal.
///      - `Type`: The type of the application (e.g., `Application`).
///      - `Categories`: Categories to classify the application.
///
/// 4. **File Creation**:
///    - Constructs a `.desktop` entry using the entered details.
///    - Creates or overwrites the `.desktop` file at the determined path.
///
fn run_cli_mode(is_global: bool, args: Vec<String>) -> std::io::Result<()> {


    // Get home directory
    let mut path = dirs::home_dir()
        .expect("Failed to get home directory");

    // Define paths
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
        println!("Name provided via flag");
        name = _name.to_string();

        println!("Name provided: {}", _name);

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
            println!("Comment provided: {}", _comment);
            comment = arg_comment_value.unwrap();
        }

        let arg_exec_path_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::EXEC_PATH)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_exec_path) = &arg_exec_path_value {
            println!("Executable path provided: {}", _exec_path);
            exec_path = arg_exec_path_value.unwrap();
        }

        let arg_icon_path_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::ICON_PATH)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_icon_path) = &arg_icon_path_value {
            println!("Icon path provided: {}", _icon_path);
            icon_path = arg_icon_path_value.unwrap();
        }

        let arg_terminal_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::TERMINAL_APP)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_terminal_app) = &arg_terminal_value {
            println!("Terminal provided: {}", _terminal_app);
            terminal_app = arg_terminal_value.unwrap();
        }

        let arg_app_type_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::APP_TYPE)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_app_type) = &arg_app_type_value {
            println!("App type provided: {}", _app_type);
            app_type = arg_app_type_value.unwrap();
        }

        let arg_categories_value: Option<String> = args.iter()
            .position(|arg: &String| arg == flags::CATEGORIES)
            .and_then(|index| args.get(index + 1))
            .map(|value: &String| value.to_string());

        if let Some(_categories) = &arg_categories_value {
            println!("Categories provided: {}", _categories);
            categories = arg_categories_value.unwrap();
        }


    } else {
        // --name has not been used, thus details will need to be provided by user

        println!("Ask user for details");

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

/// Runs the application in GUI mode and processes user input to create a desktop entry if applicable.
///
/// This function sets up and runs a graphical user interface (GUI) application, waits for user input,
/// and processes the results after the GUI window is closed. The expected behavior is as follows:
///
/// - The GUI initializes using the GTK framework, with the application identifier `com.catley.createdesktopfile`.
/// - The application makes use of shared state (`AppState`) for storing user input, which is synchronized with the GUI using `Arc` and `Mutex`.
/// - Once the user submits the form and the GUI is closed, the application checks the submitted data. If the "name" field is non-empty,
///   it assumes the form submission was successful and proceeds to create a `.desktop` entry file for the application.
///
/// # Workflow:
/// 1. Build and run the GTK application.
/// 2. After the GUI window is closed:
///    - If the `name` field of `AppState` is not empty:
///      - Determine the local directory for desktop entry installation (typically in the user's home directory under `.local/share/applications`).
///      - Construct a `.desktop` file using user-provided data such as `name`, `comment`, and file paths.
///      - Write the `.desktop` file to the determined path.
///      - If successful, a success message is printed to indicate where the file has been saved.
///
/// # Errors:
/// - This function returns an error if:
///   - The application fails to retrieve the user's home directory.
///   - The `.desktop` entry file cannot be created or written to the filesystem.
///   - Any unexpected issues arise when locking or accessing shared state.
///
/// # Returns:
/// - Returns `std::io::Result<()>`, where an `Ok(())` indicates that the function executed successfully,
///   and any errors indicate file-related or state-related problems during the execution.
///
/// # Example:
/// ```no_run
/// fn main() {
///     if let Err(e) = run_gui_mode() {
///         eprintln!("Error: {}", e);
///     }
/// }
/// ```
///
/// Note: This function assumes that the `desktop_entry` module and `AppState` struct are available and properly implemented.
///
/// # Dependencies:
/// - Requires the `gtk` crate for GUI functionality.
/// - Requires the `dirs` crate to determine the user's home directory.
/// - Requires file manipulation capabilities via `std::fs::File`.
///
/// # See Also:
/// - `build_ui` function: Used to set up the application's GUI.
fn run_gui_mode() -> std::io::Result<()> {
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
        
        path.push(".local/share/applications/"); // GUI mode always uses local installation
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

/// Checks if the current operating system is supported and panics if it is not.
///
/// # Parameters
/// - `supported_oses`: A vector containing strings that represent the supported operating systems.
/// - `os`: A reference to a string slice representing the current operating system.
///
/// # Behavior
/// This function compares the given `os` against the list of supported operating systems in `supported_oses`.
/// If the `os` is not found in the list:
/// - It prints a list of supported operating systems to the console.
/// - It then triggers a panic to terminate the program execution, indicating that the OS is unsupported.
///
/// If the `os` is supported, the function does nothing and the program continues execution.
///
/// # Example
/// ```rust
/// let supported = vec!["Windows", "Linux", "macOS"];
/// let current_os = "Linux";
///
/// break_here_if_os_not_supported(supported, &current_os);
/// ```
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

/// Builds the user interface for a "Desktop Entry Generator" application.
///
/// This function creates a GTK application window containing a grid layout
/// with labels, text entry fields, and a generate button. The fields allow
/// the user to input details for creating a desktop entry file (`.desktop`).
/// When the "Generate" button is clicked, the entered data is saved to a
/// `.desktop` file in the user's local application directory.
///
/// # Parameters
/// - `app`: A reference to the GTK [`Application`](gtk::Application) instance.
/// - `state`: A shared, thread-safe mutable reference (wrapped in an `Arc<Mutex>`)
///   to the application's state, which holds the data for the `.desktop` file.
///
/// # UI Components
/// The window contains:
/// - Labels and corresponding text entry fields for the following:
///   - **Name**: Name of the application.
///   - **Comment**: A comment or description of the application.
///   - **Executable Path**: Path to the application's executable.
///   - **Icon Path**: Path to the icon file for the application.
///   - **Terminal App?**: A flag (yes/no) for whether the application should run in a terminal.
///   - **Categories**: Categories to classify the application (e.g., "Utility;Development").
/// - A "Generate" button that triggers the file creation.
///
/// # File Creation
/// When the "Generate" button is clicked:
/// - The values from the text entry fields are saved to the shared `state`.
/// - A `.desktop` file is generated using the current state and placed
///   in the `.local/share/applications/` directory within the user's home directory.
/// - A success message is displayed in a dialog box upon successful creation.
///
/// # Behavior
/// - If the required directory structure for `.desktop` files does not exist,
///   it is created automatically.
/// - After successfully writing the `.desktop` file, all input fields are cleared.
///
/// # Example
/// ```ignore
/// let app = gtk::Application::new(
///     Some("com.example.desktop_entry_generator"),
///     gtk::ApplicationFlags::HANDLES_OPEN,
/// );
///
/// app.connect_activate(|app| {
///     // Shared application state initialization
///     let state = Arc::new(Mutex::new(AppState::default()));
///
///     // Build the UI
///     build_ui(app, &state);
/// });
///
/// app.run();
/// ```
///
/// # Dependencies
/// - **GTK**: Used to create the graphical user interface.
/// - **dirs**: To determine the user's home directory path.
/// - **std::fs**: For handling file creation and directory management.
/// - **desktop_entry**: A hypothetical module (not detailed here) for creating
///   `.desktop` files.
///
/// # Notes
/// - The `.desktop` file adheres to the Freedesktop.org Desktop Entry Specification.
/// - Ensure the `dirs` crate and GTK bindings are added as dependencies in `Cargo.toml`.
///
/// # Errors
/// - If the home directory cannot be determined, the function will panic when attempting
///   to construct the file path.
/// - If the file or directory creation fails, an error message will be logged to the
///   standard error stream.
///
/// # Related
/// For more about the `.desktop` file format, refer to:
/// [Desktop Entry Specification](https://standards.freedesktop.org/desktop-entry-spec/latest/).
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
    let info_label = Label::new(Some("This will only create .desktop files within ~/.local/share/applications"));
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

            path.push(".local/share/applications/");
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