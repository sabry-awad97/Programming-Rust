static mut STASH: &i32 = &10;

fn foo(x: &'static i32) {
    unsafe {
        STASH = x;
    }
}

static WORTH_POINTING_AT: i32 = 1000;

fn main() {
    let y = 5;
    foo(&y); // error: y has a shorter lifetime than STASH
    foo(&WORTH_POINTING_AT); // okay, WORTH_POINTING_AT is a static
}
