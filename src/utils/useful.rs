use urlencoding::encode;


/// Uses shields.io
pub fn generate_badge(label: &str, message: &str, color: &str) -> String {
    let encoded_label = encode(label);
    let encoded_message = encode(message);
    format!(
        "https://img.shields.io/badge/{}-{}-{}.svg",
        encoded_label, encoded_message, color
    )
}