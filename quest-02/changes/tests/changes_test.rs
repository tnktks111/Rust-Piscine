use changes::*;

#[test]
fn test_new() {
    let light = Light::new("living_room");

    assert_eq!(light.alias, "living_room");
    assert_eq!(light.brightness, 0);
}

#[test]
fn test_change_brightness() {
    let mut lights = [
        Light::new("living_room"),
        Light::new("bedroom"),
        Light::new("rest_room"),
    ];

    change_brightness(&mut lights, "living_room", 200);

    assert_eq!(lights[0].brightness, 200);
    assert_eq!(lights[1].brightness, 0);
    assert_eq!(lights[2].brightness, 0);
}

#[test]
fn test_not_found() {
    let mut lights = [Light::new("bedroom")];

    change_brightness(&mut lights, "unknown", 100);

    assert_eq!(lights[0].brightness, 0);
}
