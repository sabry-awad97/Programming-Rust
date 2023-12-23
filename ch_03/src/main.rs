use std::mem;

fn main() {
    let size = mem::size_of_val(&());
    assert_eq!(size, 0);

    let size = mem::size_of_val(&(1, 2, 3));
    assert_eq!(size, 3 * mem::size_of::<i32>());
}
