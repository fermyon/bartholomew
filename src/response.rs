pub fn not_found(route: String, body: String) {
    eprintln!("Not Found: {}", route);
    println!("Content-Type: text/html; charset=utf-8");
    println!("Status: 404 Not Found\n");
    println!("{}", body);
}

pub fn internal_server_error(body: String) {
    println!("Content-Type: text/plain");
    println!("Status: 500 Internal Server Error\n");
    println!("{}", body);
}

// This function is getting a little gnarly.
pub fn send_result(route: String, body: String, content_type: String, status_opt: Option<String>) {
    eprintln!("responded: {}", route);

    // Intentionally do not override the Wagi default behavior with a default Bartholomew message.
    if let Some(status) = status_opt {
        println!("Status: {}", status);
    }
    println!("Content-Type: {}\n", content_type);
    println!("{}", body);
}

pub fn send_redirect(route: String, location: String, status: String) {
    eprintln!("redirected {} to {} (Code: {})", route, &location, &status);
    println!("Status: {}\nLocation: {}\n", status, location)
}
