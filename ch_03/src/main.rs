use std::mem;

fn main() {
    let mut t = (1, 2, 3);

    let size = mem::size_of_val(&t);
    assert_eq!(size, 3 * mem::size_of::<i32>());

    let align = mem::align_of_val(&t);
    assert_eq!(align, mem::align_of::<i32>());

    let mut u = (4, 5, 6);
    mem::swap(&mut t, &mut u);
    assert_eq!(t, (4, 5, 6));
    assert_eq!(u, (1, 2, 3));
}
