use mobs::*;
use std::collections::{HashMap, HashSet};

fn make_mob(name: &str, wealth: u64, city: &str) -> Mob {
    Mob {
        name: name.to_string(),
        boss: Boss::new(name, 50),
        members: HashMap::new(),
        cities: HashSet::from([city.to_string()]),
        wealth,
    }
}

#[test]
fn test_recruit() {
    let mut mob = make_mob("A", 100, "Tokyo");

    mob.recruit(("Alice", 20));
    mob.recruit(("Bob", 30));

    assert_eq!(mob.members.len(), 2);
    assert_eq!(mob.members["Alice"].age, 20);
    assert_eq!(mob.members["Alice"].role, Role::Associate);
    assert_eq!(mob.members["Bob"].role, Role::Associate);
}

#[test]
fn test_promotion() {
    let mut mob = make_mob("A", 100, "Tokyo");
    mob.recruit(("Alice", 20));

    let member = mob.members.get_mut("Alice").unwrap();

    member.get_promotion();
    assert_eq!(member.role, Role::Soldier);

    member.get_promotion();
    assert_eq!(member.role, Role::Caporegime);

    member.get_promotion();
    assert_eq!(member.role, Role::Underboss);
}

#[test]
#[should_panic]
fn test_promotion_underboss() {
    let mut mob = make_mob("A", 100, "Tokyo");
    mob.recruit(("Alice", 20));

    let member = mob.members.get_mut("Alice").unwrap();

    for _ in 0..4 {
        member.get_promotion();
    }
}

#[test]
fn test_attack_remove_youngest() {
    let mut a = make_mob("A", 100, "Tokyo");
    let mut b = make_mob("B", 200, "Osaka");

    // A: combat score = 4
    for i in 0..4 {
        a.recruit((&format!("A{i}"), 30));
    }

    // B: combat score = 3
    b.recruit(("Bob", 20));
    b.recruit(("Ben", 20));
    b.recruit(("Charlie", 40));

    a.attack(&mut b);

    // 最年少の2人が削除される
    assert_eq!(b.members.len(), 1);
    assert!(b.members.contains_key("Charlie"));
    assert!(!b.members.contains_key("Bob"));
    assert!(!b.members.contains_key("Ben"));

    // 全滅していないので財産・都市はそのまま
    assert_eq!(a.wealth, 100);
    assert_eq!(b.wealth, 200);
    assert!(b.cities.contains("Osaka"));
}

#[test]
fn test_attack_draw() {
    let mut a = make_mob("A", 100, "Tokyo");
    let mut b = make_mob("B", 200, "Osaka");

    a.recruit(("Alice", 20));
    a.recruit(("Bob", 30));

    b.recruit(("Charlie", 25));
    b.recruit(("David", 35));

    // 同点なら攻撃側 A が負ける
    a.attack(&mut b);

    assert_eq!(a.members.len(), 1);
    assert!(!a.members.contains_key("Alice"));
    assert!(a.members.contains_key("Bob"));

    assert_eq!(b.members.len(), 2);
    assert_eq!(a.wealth, 100);
    assert_eq!(b.wealth, 200);
}

#[test]
fn test_attack_combat_score() {
    let mut a = make_mob("A", 100, "Tokyo");
    let mut b = make_mob("B", 200, "Osaka");

    // A: Underboss = 4
    a.recruit(("Alice", 30));

    let member = a.members.get_mut("Alice").unwrap();
    for _ in 0..3 {
        member.get_promotion();
    }

    // B: Associate x 3 = 3
    b.recruit(("Bob", 20));
    b.recruit(("Charlie", 25));
    b.recruit(("David", 40));

    a.attack(&mut b);

    assert_eq!(a.members.len(), 1);
    assert_eq!(b.members.len(), 2);
    assert!(!b.members.contains_key("Bob"));
}

#[test]
fn test_attack_take_cities_and_wealth() {
    let mut a = make_mob("A", 100, "Tokyo");
    let mut b = make_mob("B", 200, "Osaka");

    a.recruit(("Alice", 30));
    a.recruit(("Bob", 35));

    b.recruit(("Charlie", 20));

    a.attack(&mut b);

    // B が全滅
    assert!(b.members.is_empty());

    // A が B の財産と都市を取得
    assert_eq!(a.wealth, 300);
    assert_eq!(b.wealth, 0);

    assert!(a.cities.contains("Tokyo"));
    assert!(a.cities.contains("Osaka"));
    assert!(b.cities.is_empty());
}

#[test]
fn test_attack_attacker_loses_everything() {
    let mut a = make_mob("A", 100, "Tokyo");
    let mut b = make_mob("B", 200, "Osaka");

    a.recruit(("Alice", 20));

    b.recruit(("Bob", 30));
    b.recruit(("Charlie", 35));

    a.attack(&mut b);

    assert!(a.members.is_empty());
    assert_eq!(a.wealth, 0);
    assert_eq!(b.wealth, 300);

    assert!(a.cities.is_empty());
    assert!(b.cities.contains("Tokyo"));
    assert!(b.cities.contains("Osaka"));
}

#[test]
fn test_steal() {
    let mut a = make_mob("A", 100, "Tokyo");
    let mut b = make_mob("B", 200, "Osaka");

    // 通常の窃盗
    a.steal(&mut b, 50);

    assert_eq!(a.wealth, 150);
    assert_eq!(b.wealth, 150);

    // 所持金より多く盗もうとする
    a.steal(&mut b, 300);

    assert_eq!(a.wealth, 300);
    assert_eq!(b.wealth, 0);
}

#[test]
fn test_conquer_city() {
    let mut a = make_mob("A", 100, "Tokyo");
    let b = make_mob("B", 200, "Osaka");
    let c = make_mob("C", 300, "Kyoto");

    let others = [&b, &c];

    // 誰も所有していない都市
    a.conquer_city(&others, "Nagoya".to_string());
    assert!(a.cities.contains("Nagoya"));

    // 他の Mob が所有する都市
    a.conquer_city(&others, "Osaka".to_string());
    assert!(!a.cities.contains("Osaka"));

    a.conquer_city(&others, "Kyoto".to_string());
    assert!(!a.cities.contains("Kyoto"));

    // 重複追加しても HashSet の要素数は変わらない
    a.conquer_city(&others, "Nagoya".to_string());
    assert_eq!(a.cities.len(), 2);
}
