struct Person {
    name: Option<String>,
    birth: i32,
}

fn main() {
    let mut composers = vec![Person {
        name: Some("Palestrina".to_string()),
        birth: 1525,
    }];
    let first_name = composers[0].name.take(); // first_name is a Option<String>
    println!("{:?}", composers[0].name); // composers[0].name is None
}
