use strsim::levenshtein;

fn main() {
    let string1 = "kitten";
    let string2 = "sitting";

    // Calculate Levenshtein distance between the strings
    let distance = levenshtein(string1, string2);

    println!("Levenshtein distance: {}", distance);
}
