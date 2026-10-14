use question_mark::*;

#[test]
fn test_get_fourth_layer() {
    let a = One {
        first_layer: Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: Some(1000),
                }),
            }),
        }),
    };

    assert_eq!(a.get_fourth_layer(), Some(1000));
}

#[test]
fn test_none() {
    let a = One { first_layer: None };

    assert_eq!(a.get_fourth_layer(), None);

    let b = One {
        first_layer: Some(Two { second_layer: None }),
    };

    assert_eq!(b.get_fourth_layer(), None);

    let c = One {
        first_layer: Some(Two {
            second_layer: Some(Three { third_layer: None }),
        }),
    };

    assert_eq!(c.get_fourth_layer(), None);

    let d = One {
        first_layer: Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four { fourth_layer: None }),
            }),
        }),
    };

    assert_eq!(d.get_fourth_layer(), None);
}
