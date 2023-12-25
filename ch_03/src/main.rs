use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let text = "a̐éö̲\u{305}"; // A string with various Unicode characters

    // Break the string into grapheme clusters
    let clusters: Vec<&str> = text.graphemes(true).collect();

    println!("Grapheme Clusters: {:?}", clusters);
}
