use geo::geometry::{Coord, Rect};

fn main() {
    let rect = Rect::new(
        Coord { x: 0., y: 4. },
        Coord { x: 3., y: 10. },
    );

    println!("{rect:?}");
}
