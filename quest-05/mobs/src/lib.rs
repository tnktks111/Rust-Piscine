mod boss;
mod member;

pub use boss::Boss;
pub use member::{Member, Role};
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: HashMap<String, Member>,
    pub cities: HashSet<String>,
    pub wealth: u64,
}

impl Mob {
    pub fn recruit(&mut self, (name, age): (&str, u32)) {
        let member: Member = Member {
            age: age,
            role: Role::Associate,
        };
        self.members.insert(name.to_string(), member);
    }

    fn total_combat_score(&self) -> u32 {
        self.members
            .values()
            .map(|member| member.combat_score())
            .sum::<u32>()
    }

    fn remove_youngest_members(&mut self) {
        if let Some(min_age) = self.members.values().map(|member| member.age).min() {
            self.members.retain(|_, member| member.age != min_age);
        }
    }

    fn take_cities_and_wealth_of(&mut self, other: &mut Mob) {
        self.cities.extend(other.cities.drain());
        other.cities = HashSet::new();
        self.wealth += other.wealth;
        other.wealth = 0;
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_combat_score = self.total_combat_score();
        let other_combat_score = other.total_combat_score();
        if self_combat_score <= other_combat_score {
            self.remove_youngest_members();
            if self.members.len() == 0 {
                other.take_cities_and_wealth_of(self);
            }
        } else {
            other.remove_youngest_members();
            if other.members.len() == 0 {
                self.take_cities_and_wealth_of(other);
            }
        }
    }

    pub fn steal(&mut self, other: &mut Mob, amount: u64) {
        let stolen = amount.min(other.wealth);

        self.wealth += stolen;
        other.wealth -= stolen;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city: String) {
        if !other_mobs.iter().any(|mob| mob.cities.contains(&city)) {
            self.cities.insert(city);
        }
    }
}
