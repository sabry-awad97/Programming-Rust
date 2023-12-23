fn tuple_length<T: std::fmt::Debug + Sized>(tuple: &T) -> usize {
    let s = format!("{:#?}", tuple);
    println!("{:#?}", s);
    match s.contains(',') {
        true => {
            let parts = s.matches(',').collect::<Vec<&str>>();
            parts.len()
        }
        _ => 1,
    }
}

fn main() {
    let t = (1, "hello", std::f64::consts::PI);
    println!("Number of tuple elements: {}", tuple_length(&t));
}
