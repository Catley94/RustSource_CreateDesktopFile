pub fn display_help_information(args: Vec<String>) {
    println!("CreateDesktopFile v{}", env!("CARGO_PKG_VERSION"));
    println!("CreateDesktopFile is a simple tool to create .desktop files for Linux. \n\
    By default it will open a GUI app, however this can also run in Terminal by specifying the below flags/arguments");
    println!("Usage: {} [--cli | --global | --local]", args[0]);
    println!("Options:");
    println!("  --cli       Runs in CLI mode, defaults to --local");
    println!("  --local     Install .desktop file locally in ~/.local/share/applications/");
    println!("  --global    Install .desktop file globally in /usr/share/applications/");
    println!("  --help      Show this help message");
}