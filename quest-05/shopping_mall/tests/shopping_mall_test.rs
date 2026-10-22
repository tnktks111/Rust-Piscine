use shopping_mall::*;
use std::collections::HashMap;

fn sample_mall() -> Mall {
    Mall {
        name: "Test Mall".to_string(),
        guards: HashMap::from([(
            "Guard A".to_string(),
            Guard {
                age: 30,
                years_experience: 5,
            },
        )]),
        floors: HashMap::from([
            (
                "Ground".to_string(),
                Floor {
                    size_limit: 400,
                    stores: HashMap::from([(
                        "Small".to_string(),
                        Store {
                            square_meters: 150,
                            employees: HashMap::from([
                                (
                                    "Alice".to_string(),
                                    Employee {
                                        age: 25,
                                        working_hours: (9, 19),
                                        salary: 1000.0,
                                    },
                                ),
                                (
                                    "Bob".to_string(),
                                    Employee {
                                        age: 30,
                                        working_hours: (9, 18),
                                        salary: 1200.0,
                                    },
                                ),
                            ]),
                        },
                    )]),
                },
            ),
            (
                "Upper".to_string(),
                Floor {
                    size_limit: 400,
                    stores: HashMap::from([(
                        "Large".to_string(),
                        Store {
                            square_meters: 320,
                            employees: HashMap::from([
                                (
                                    "Carol".to_string(),
                                    Employee {
                                        age: 28,
                                        working_hours: (20, 6),
                                        salary: 1200.0,
                                    },
                                ),
                                (
                                    "Dan".to_string(),
                                    Employee {
                                        age: 35,
                                        working_hours: (10, 12),
                                        salary: 800.0,
                                    },
                                ),
                            ]),
                        },
                    )]),
                },
            ),
        ]),
    }
}

#[test]
fn test_biggest_store() {
    let mall = sample_mall();

    let (name, store) = biggest_store(&mall);

    assert_eq!(name, "Large");
    assert_eq!(store.square_meters, 320);
}

#[test]
fn test_highest_paid_employee() {
    let mall = sample_mall();

    let result = highest_paid_employee(&mall);

    // 同率1位の2人を返す
    assert_eq!(result.len(), 2);

    let mut names: Vec<&str> = result.iter().map(|(name, _)| *name).collect();

    names.sort_unstable();

    assert_eq!(names, vec!["Bob", "Carol"]);

    for (_, employee) in result {
        assert_eq!(employee.salary, 1200.0);
    }
}

#[test]
fn test_nbr_of_employees() {
    let mall = sample_mall();

    // 従業員4人 + 警備員1人
    assert_eq!(nbr_of_employees(&mall), 5);
}

#[test]
fn test_check_for_securities() {
    let mut mall = sample_mall();

    let available = HashMap::from([
        (
            "Guard B".to_string(),
            Guard {
                age: 30,
                years_experience: 3,
            },
        ),
        (
            "Guard C".to_string(),
            Guard {
                age: 40,
                years_experience: 10,
            },
        ),
        (
            "Guard D".to_string(),
            Guard {
                age: 25,
                years_experience: 2,
            },
        ),
    ]);

    // 総面積470m²
    // 必要人数 ceil(470 / 200) = 3
    // 現在1人なので2人追加
    check_for_securities(&mut mall, available);

    assert_eq!(mall.guards.len(), 3);
    assert!(mall.guards.contains_key("Guard A"));
}

#[test]
fn test_check_for_securities_no_hire() {
    let mut mall = sample_mall();

    mall.guards.insert(
        "Guard B".to_string(),
        Guard {
            age: 30,
            years_experience: 3,
        },
    );

    mall.guards.insert(
        "Guard C".to_string(),
        Guard {
            age: 40,
            years_experience: 10,
        },
    );

    // すでに3人いるため追加不要
    let available = HashMap::from([(
        "Guard D".to_string(),
        Guard {
            age: 25,
            years_experience: 2,
        },
    )]);

    check_for_securities(&mut mall, available);

    assert_eq!(mall.guards.len(), 3);
    assert!(!mall.guards.contains_key("Guard D"));
}

#[test]
fn test_cut_or_raise() {
    let mut mall = sample_mall();

    cut_or_raise(&mut mall);

    let small = &mall.floors["Ground"].stores["Small"];
    let large = &mall.floors["Upper"].stores["Large"];

    let alice = small.employees["Alice"].salary;
    let bob = small.employees["Bob"].salary;
    let carol = large.employees["Carol"].salary;
    let dan = large.employees["Dan"].salary;

    // Alice: 10時間 → 10%増額
    assert!((alice - 1100.0).abs() < 1e-9);

    // Bob: 9時間 → 10%減額
    assert!((bob - 1080.0).abs() < 1e-9);

    // Carol: 20時〜翌6時 = 10時間 → 増額
    assert!((carol - 1320.0).abs() < 1e-9);

    // Dan: 2時間 → 減額
    assert!((dan - 720.0).abs() < 1e-9);

    // 警備員は変更対象外
    assert_eq!(mall.guards.len(), 1);
}
