use serde_json::Value;

/// Terminal service module for Nebula Terminal
pub mod terminal_service {
    use super::*;

    /// Greets a user with a personalized message
    pub fn greet_user(name: &str) -> String {
        format!("Hello, {}! Welcome to Nebula Terminal!", name)
    }

    /// Returns terminal information
    pub fn get_terminal_info() -> Value {
        serde_json::json!({
            "name": "Nebula Terminal",
            "version": "0.1.0",
            "status": "ready"
        })
    }

    /// Validates terminal configuration
    pub fn validate_config(config: &Value) -> bool {
        config.get("name").is_some() && config.get("version").is_some()
    }
}

/// Utility functions for the terminal
pub mod utils {
    /// Formats a command for execution
    pub fn format_command(cmd: &str, args: &[&str]) -> String {
        if args.is_empty() {
            cmd.to_string()
        } else {
            format!("{} {}", cmd, args.join(" "))
        }
    }

    /// Checks if a string is a valid command
    pub fn is_valid_command(cmd: &str) -> bool {
        !cmd.trim().is_empty() && !cmd.contains('\0')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_user() {
        let result = terminal_service::greet_user("Alice");
        assert_eq!(result, "Hello, Alice! Welcome to Nebula Terminal!");
    }

    #[test]
    fn test_get_terminal_info() {
        let info = terminal_service::get_terminal_info();
        assert_eq!(info["name"], "Nebula Terminal");
        assert_eq!(info["version"], "0.1.0");
        assert_eq!(info["status"], "ready");
    }

    #[test]
    fn test_validate_config_valid() {
        let config = serde_json::json!({
            "name": "Test Terminal",
            "version": "1.0.0"
        });
        assert!(terminal_service::validate_config(&config));
    }

    #[test]
    fn test_validate_config_invalid() {
        let config = serde_json::json!({
            "name": "Test Terminal"
        });
        assert!(!terminal_service::validate_config(&config));
    }

    #[test]
    fn test_format_command_no_args() {
        let result = utils::format_command("ls", &[]);
        assert_eq!(result, "ls");
    }

    #[test]
    fn test_format_command_with_args() {
        let result = utils::format_command("ls", &["-la", "/home"]);
        assert_eq!(result, "ls -la /home");
    }

    #[test]
    fn test_is_valid_command_valid() {
        assert!(utils::is_valid_command("ls"));
        assert!(utils::is_valid_command("echo hello"));
    }

    #[test]
    fn test_is_valid_command_invalid() {
        assert!(!utils::is_valid_command(""));
        assert!(!utils::is_valid_command("   "));
        assert!(!utils::is_valid_command("ls\0"));
    }
} 