// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// I AM DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref a) => println!("Co-ordinates are {},{} ", a.x, a.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
