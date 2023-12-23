use std::str::FromStr;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 2 {
            return Err(format!("Error parsing point: {}", s));
        }
        let x = coords[0].parse::<i32>().map_err(|e| e.to_string())?;
        let y = coords[1].parse::<i32>().map_err(|e| e.to_string())?;
        Ok(Self { x, y })
    }
}

fn main() {
    let p: Point = "1,2".parse().unwrap();
    println!("{:?}", p); // prints "Point { x: 1, y: 2 }"

    let q: Result<Point, String> = "3,4,5".parse();
    println!("{:?}", q); // prints "Err("Error parsing point: 3,4,5")"
}
