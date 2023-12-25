use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "Hello, world! 😀";
    for (i, grapheme) in s.grapheme_indices(true) {
        println!("Grapheme cluster at index {}: {}", i, grapheme);
    }
}
