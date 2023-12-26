fn main() {
    let mut value = 5; // Creating a mutable variable 'value'

    {
        let reference = &value; // Creating an immutable reference to 'value'
        println!("Immutable reference: {}", reference);
    } // The immutable reference 'reference' goes out of scope here

    let another_reference = &mut value; // Creating a mutable reference to 'value'
    *another_reference += 10; // Modifying the value through the mutable reference
    println!("Modified value: {}", value);
}
