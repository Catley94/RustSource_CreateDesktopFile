#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;
    use crate::{break_here_if_os_not_supported, desktop_entry, path};
    use crate::modes::RUN_CLI;

    // Helper function to setup a temporary directory for tests
    fn setup_test_dir() -> (tempfile::TempDir, PathBuf) {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let test_path = temp_dir.path().to_path_buf();
        (temp_dir, test_path)
    }

    #[test]
    fn test_cli_basic_local_installation() {
        let (temp_dir, test_path) = setup_test_dir();

        println!("temp_dir: {}, test_path: {}", temp_dir.path().display(), test_path.display());
        
        // Simulate CLI arguments
        let args = vec![
            "CreateDesktopFile".to_string(),
            "--local".to_string(),
            "--name".to_string(),
            "TestApp".to_string(),
            "--comment".to_string(),
            "Test Application".to_string(),
            "--exec-path".to_string(),
            "/usr/bin/test".to_string(),
            "--icon-path".to_string(),
            "/usr/share/icons/test.png".to_string(),
            "--terminal-app".to_string(),
            "false".to_string(),
            "--app-type".to_string(),
            "Application".to_string(),
            "--categories".to_string(),
            "Development;".to_string(),
        ];
        println!("{:?}", args);


        println!("Running cli mode");
        // Run CLI mode with test arguments
        let result = RUN_CLI(false, args, test_path.to_str().unwrap(), "");

        println!("Getting result");
        assert!(result.is_ok());
        println!("Result OK");

        // Verify the .desktop file was created with correct content
        let desktop_file_path = test_path.join("TestApp.desktop");
        println!("Desktop file path: {}", desktop_file_path.display());
        assert!(desktop_file_path.exists());
        println!("Desktop File Exists");
        
        let content = fs::read_to_string(desktop_file_path)
            .expect("Failed to read desktop file");
        
        assert!(content.contains("[Desktop Entry]"));
        assert!(content.contains("Name=TestApp"));
        assert!(content.contains("Comment=Test Application"));
        assert!(content.contains("Exec=/usr/bin/test"));
        assert!(content.contains("Icon=/usr/share/icons/test.png"));
        assert!(content.contains("Terminal=false"));
        assert!(content.contains("Type=Application"));
        assert!(content.contains("Categories=Development;"));
    }

    #[test]
    #[should_panic(expected = "Need to specify --name")]
    fn test_cli_missing_name_flag() {
        let args = vec![
            "CreateDesktopFile".to_string(),
            // "--local".to_string(),
            "--comment".to_string(),
            "Test Application".to_string(),
        ];
        
        RUN_CLI(false, args, path::LOCAL_SHARE_APPLICATIONS, path::GLOBAL_SHARE_APPLICATIONS).unwrap();

        // TODO: This is not failing or panicing
    }

    #[test]
    fn test_cli_with_spaces_in_comment() {
        let (temp_dir, test_path) = setup_test_dir();
        
        let args = vec![
            "CreateDesktopFile".to_string(),
            "--local".to_string(),
            "--name".to_string(),
            "TestApp".to_string(),
            "--comment".to_string(),
            "This is a test application with spaces".to_string(),
            "--exec-path".to_string(),
            "/usr/bin/test".to_string(),
        ];

        let result = RUN_CLI(false, args, test_path.to_str().unwrap(), test_path.to_str().unwrap());
        assert!(result.is_ok());
        
        let content = fs::read_to_string(test_path.join("TestApp.desktop"))
            .expect("Failed to read desktop file");
        
        assert!(content.contains("Comment=This is a test application with spaces"));
    }

    #[test]
    fn test_desktop_entry_generation() {
        let entry = desktop_entry::DesktopEntry::new(
            "TestApp".to_string(),
            "Test Comment".to_string(),
            "/usr/bin/test".to_string(),
            "/usr/share/icons/test.png".to_string(),
            "false".to_string(),
            "Application".to_string(),
            "Development;".to_string(),
        );

        let entry_string = entry.to_string();
        assert!(entry_string.contains("[Desktop Entry]"));
        assert!(entry_string.contains("Name=TestApp"));
        assert!(entry_string.contains("Exec=/usr/bin/test"));
        assert!(entry_string.contains("Type=Application"));
    }

    #[test]
    fn test_supported_os_check() {
        let supported_oses = vec!["linux"];
        
        // This should not panic on Linux
        break_here_if_os_not_supported(supported_oses.clone(), &"linux");
        
        // Test with unsupported OS
        let result = std::panic::catch_unwind(|| {
            break_here_if_os_not_supported(supported_oses, &"windows");
        });
        assert!(result.is_err());
    }
}