use speed_transformation::*;

#[test]
fn test_speed_transformation() {
    let km_h = 100.0;
    assert_eq!(27.77777777777778, km_per_hour_to_meters_per_second(km_h));
}
