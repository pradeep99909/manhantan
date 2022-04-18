mod formula;
fn main() {
    let mut point1 = formula::structure::Point{
        x: 10,
        y: 10
    };

    let mut point2 = formula::structure::Point{
        x: 2,
        y: 2
    };

    println!("{}", formula::formula::nearest_distance(point1, point2));
}
