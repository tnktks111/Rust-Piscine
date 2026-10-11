use circle::*;

#[test]
fn test_circle() {
    let c = Circle::new(0.0, 0.0, 5.0);

    assert_eq!(c.diameter(), 10.0);
    assert_eq!(c.area(), 25.0 * std::f64::consts::PI);
}

#[test]
fn test_distance() {
    let a = Point(0.0, 0.0);
    let b = Point(3.0, 4.0);

    assert_eq!(a.distance(b), 5.0);
}

#[test]
fn test_intersect() {
    let c = Circle::new(0.0, 0.0, 5.0);

    // 2点で交差
    assert!(c.intersect(Circle::new(6.0, 0.0, 5.0)));

    // 外接
    assert!(c.intersect(Circle::new(10.0, 0.0, 5.0)));

    // 内接
    assert!(c.intersect(Circle::new(3.0, 0.0, 2.0)));

    // 離れている
    assert!(!c.intersect(Circle::new(20.0, 0.0, 5.0)));

    // 完全に内包されている
    assert!(!c.intersect(Circle::new(0.0, 0.0, 2.0)));

    // 同一の円
    assert!(c.intersect(c));
}