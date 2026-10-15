mod areas_volumes;
use areas_volumes::*;
pub use areas_volumes::{GeometricalShapes, GeometricalVolumes};

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area_limit = rectangle_area(x, y) as f64;
    let one_area: f64 = match kind {
        GeometricalShapes::Circle => circle_area(a),
        GeometricalShapes::Rectangle => rectangle_area(a, b) as f64,
        GeometricalShapes::Square => square_area(a) as f64,
        GeometricalShapes::Triangle => triangle_area(a, b),
    };
    one_area * times as f64 <= area_limit
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let volume_limit = parallelepiped_volume(x, y, z) as f64;
    let one_volume: f64 = match kind {
        GeometricalVolumes::Cone => cone_volume(a, b),
        GeometricalVolumes::Cube => cube_volume(a) as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) as f64,
        GeometricalVolumes::Sphere => sphere_volume(a),
        GeometricalVolumes::TriangularPyramid => triangular_pyramid_volume(a as f64, b),
    };
    one_volume * times as f64 <= volume_limit
}
