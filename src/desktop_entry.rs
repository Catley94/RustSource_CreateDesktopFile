pub struct DesktopEntry {
    name: String,
    comment: String,
    exec_path: String,
    icon_path: String,
    terminal_app: String,
    app_type: String,
    categories: String,
}

impl DesktopEntry {
    pub fn new(
        name: String,
        comment: String,
        exec_path: String,
        icon_path: String,
        terminal_app: String,
        app_type: String,
        categories: String,

    ) -> Self {
        DesktopEntry {
            name: name.to_string(),
            comment: comment.to_string(),
            exec_path: exec_path.to_string(),
            icon_path: icon_path.to_string(),
            terminal_app: terminal_app.to_string(),
            app_type: app_type.to_string(),
            categories: categories.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "[Desktop Entry]\n\
            Name={}\n\
            Comment={}\n\
            Exec={}\n\
            Icon={}\n\
            Terminal={}\n\
            Type={}\n\
            Categories={}",

            self.name.trim(),
            self.comment.trim(),
            self.exec_path.trim(),
            self.icon_path.trim(),
            self.terminal_app.trim(),
            self.app_type.trim(),
            self.categories.trim(),
        )
    }
}