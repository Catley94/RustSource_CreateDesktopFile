use std::io;

pub fn ask_user_to_fill_in_details(mut name: &mut String, mut comment: &mut String, mut exec_path: &mut String, mut icon_path: &mut String, mut terminal_app: &mut String, mut app_type: &mut String, mut categories: &mut String) {
    println!("Enter the name of the application:");
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read name");

    println!("Enter the comments for the application:");
    io::stdin()
        .read_line(&mut comment)
        .expect("Failed to read comment");

    println!("Enter the path to the executable:");
    io::stdin()
        .read_line(&mut exec_path)
        .expect("Failed to read exec path");

    println!("Enter the path to the icon:");
    io::stdin()
        .read_line(&mut icon_path)
        .expect("Failed to read icon path");

    println!("Terminal app? (true/false):");
    io::stdin()
        .read_line(&mut terminal_app)
        .expect("Failed to read terminal app");

    println!("Enter the type of application: (ex: Application)");
    io::stdin()
        .read_line(&mut app_type)
        .expect("Failed to read app type");

    println!("Enter the categories for the application: (ex: Development;)");
    io::stdin()
        .read_line(&mut categories)
        .expect("Failed to read categories");
}