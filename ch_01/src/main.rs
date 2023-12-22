fn main() {
    let mut a = [0u64; 1]; // Declaring an array 'a' of length 1

    // Attempting to assign a value to an out-of-bounds index would result in a compile-time error
    // a[3] = 0x7ffff7b36ceb;

    // Uncommenting the line above would cause a compile-time error due to out-of-bounds access
}
