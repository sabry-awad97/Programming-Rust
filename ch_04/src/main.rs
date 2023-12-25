fn main() {
    let v = vec![
        "liberté".to_string(),
        "égalité".to_string(),
        "fraternité".to_string(),
    ];
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }
    println!("{:?}", v); // error: borrow of moved value: `v`
}
