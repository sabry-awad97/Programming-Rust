use unicode_normalization::UnicodeNormalization;

fn main() {
    let input_text = "cœur"; // String with non-normalized characters

    // Normalize the string into NFC form
    let normalized_text = input_text.nfc().collect::<String>();

    println!("Normalized Text: {}", normalized_text);
}
