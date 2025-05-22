pub fn display_help_information(args: Vec<String>) {
    println!("Usage: {} [--global | --local]", args[0]);
    println!("Default: --local will be used if no argument is specified");
    println!("Options:");
    println!("  --local     Install .desktop file locally in .local/share/applications/");
    println!("  --global    Install .desktop file globally in /usr/share/applications/");
    println!("  --help      Show this help message");
}