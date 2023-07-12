use std::collections::HashMap;

/// Launches the HTTP server.
pub fn launch_server() {
    println!("Launching server...");
}

/// Launches the console interface.
pub fn launch_console(command: String, args: HashMap<String, Option<String>>) {
    println!("Launching console...");
    println!("Command: {}", command);
    println!("Args: {:?}", args);
}