use does_it_fit::*;

#[test]
fn test_area_fit() {
    assert!(!area_fit((2, 4), GeometricalShapes::Rectangle, 100, (2, 1)));

    assert!(area_fit((5, 5), GeometricalShapes::Triangle, 3, (5, 3)));
}

#[test]
fn test_volume_fit() {
    assert!(volume_fit(
        (5, 5, 5),
        GeometricalVolumes::Sphere,
        3,
        (2, 0, 0)
    ));

    assert!(volume_fit(
        (5, 7, 5),
        GeometricalVolumes::Parallelepiped,
        1,
        (6, 7, 4)
    ));
}
