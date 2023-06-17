use geo::geometry::{Coord, Rect};
use geo::algorithm::BooleanOps;

fn main() {
    let rect_a = Rect::new(
        Coord { x: 0., y: 4. },
        Coord { x: 3., y: 10. },
    );

    let rect_c = Rect::new(
        Coord { x: 0., y: 4. },
        Coord { x: 3., y: 10. },
    );
    
    let rect_b = rect_a.to_polygon().difference(&rect_c.to_polygon());


    println!("{rect_a:?}\n{rect_b:?}");
}
