struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                return Some(&self.elements[i]);
            }
        }
        None
    }
}

fn main() {
    let mut table = StringTable {
        elements: Vec::new(), // Create an empty StringTable
    };
    table.elements.push("apple".to_string());
    table.elements.push("banana".to_string());
    table.elements.push("orange".to_string());

    // Search for strings starting with a specific prefix
    let prefix = "app";
    match table.find_by_prefix(prefix) {
        Some(found_string) => {
            println!("String found: {}", found_string);
            // Use the found_string reference here...
        }
        None => {
            println!("No string found with the prefix: {}", prefix);
            // Handle the case where no matching string is found...
        }
    }
}
