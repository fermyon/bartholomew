use chrono::{SecondsFormat, Utc};
use rhai::Engine;

// Creates a custom instance of the rhai enginer with helper functions
pub fn custom_rhai_engine_init() -> Engine {
    let mut rhai_engine = Engine::new();

    rhai_engine.register_fn("current_date", current_date);
    rhai_engine.register_fn("console_println", console_println);

    rhai_engine
}

// Allows access for current_date in rhai script to allow for filtering
// The specific date format is to allow for comparision inside rhai scripts
fn current_date() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}

// Allows for some debugging capability for rhai scripts by printing to stdout
// instead of having to serially print output of steps on the webpage
fn console_println(message: String) {
    eprintln!("{message}");
}
