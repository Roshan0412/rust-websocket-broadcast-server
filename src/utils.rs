use chrono::Local;

pub fn timestamp() -> String {
    Local::now().format("[%H:%M:%S]").to_string()
}
