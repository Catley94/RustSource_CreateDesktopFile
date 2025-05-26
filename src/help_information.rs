use crate::flags;
pub fn display_help_information(args: Vec<String>) {
    println!("create-desktop-file v{}", env!("CARGO_PKG_VERSION"));
    println!("create-desktop-file is a simple tool to create .desktop files for Linux. \n\
    By default it will open a GUI app, however this can also run in Terminal by specifying the below flags/arguments");
    println!("Usage: {}  [--global | --local] etc.", args[0]);
    println!("Options:");
    println!("  {}", flags::LOCAL);
    println!("      Install .desktop file locally in ~/.local/share/applications/");
    println!("  {}", flags::GLOBAL);
    println!("      Install .desktop file globally in /usr/share/applications/");
    println!("  {}", flags::NAME);
    println!("      Set the name of the .desktop file, if not used, it will ask you specifically for the details");
    println!("  {}", flags::COMMENT);
    println!("      (Requires --name) Set the comment of the .desktop file");
    println!("  {}", flags::EXEC_PATH);
    println!("      (Requires --name) Set the command to execute");
    println!("  {}", flags::ICON_PATH);
    println!("      (Requires --name) Set the icon to use");
    println!("  {}", flags::TERMINAL_APP);
    println!("      (Requires --name) Run the command in Terminal");
    println!("  {}", flags::APP_TYPE);
    println!("      (Requires --name) Set the type of the .desktop file (Application, Link, Directory, etc.)");
    println!("  {}", flags::CATEGORIES);
    println!("      (Requires --name) Set the categories of the .desktop file (Utility, Game, etc.)");
    println!("  {}", flags::VERSION);
    println!("      Show version information");
    println!("  {}", flags::HELP);
    println!("      Show this help message");
}