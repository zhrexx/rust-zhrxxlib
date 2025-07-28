use urlencoding::encode;

/// Function to generate badges
/// NOTE: Uses shields.io
pub fn generate_badge(label: &str, message: &str, color: &str) -> String {
    format!(
        "https://img.shields.io/badge/{}-{}-{}.svg",
        encode(label), encode(message), color
    )
}