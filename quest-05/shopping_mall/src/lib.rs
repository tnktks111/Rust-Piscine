mod mall;
use std::collections::HashMap;

pub use mall::*;

pub fn biggest_store(mall: &Mall) -> (&str, &Store) {
    let mut biggest_store: Option<(&str, &Store)> = None;

    for floor in mall.floors.values() {
        if let Some((name, store)) = floor
            .stores
            .iter()
            .max_by_key(|entry| entry.1.square_meters)
        {
            if biggest_store.is_none_or(|current| current.1.square_meters < store.square_meters) {
                biggest_store = Some((name, store));
            }
        }
    }
    biggest_store.expect("No stores found.")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, &Employee)> {
    let employees = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .flat_map(|store| store.employees.iter());

    let max_salary = employees
        .clone()
        .max_by(|a, b| a.1.salary.total_cmp(&b.1.salary))
        .expect("No Employees found")
        .1
        .salary;

    employees
        .filter(|(_, employee)| employee.salary == max_salary)
        .map(|(name, employee)| (name.as_str(), employee))
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len()
        + mall
            .floors
            .values()
            .flat_map(|floor| floor.stores.values())
            .map(|store| store.employees.len())
            .sum::<usize>()
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let total_area = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum::<u64>();
    let required = total_area.div_ceil(200) as usize;
    let needed = required.saturating_sub(mall.guards.len());

    for (name, guard) in available_guards.into_iter().take(needed) {
        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.floors
        .values_mut()
        .flat_map(|floor| floor.stores.values_mut())
        .flat_map(|store| store.employees.values_mut())
        .for_each(|employee| {
            let hours = (24 + employee.working_hours.1 - employee.working_hours.0) % 24;
            let diff = employee.salary * 0.1;

            if hours >= 10 {
                employee.raise(diff);
            } else {
                employee.cut(diff);
            }
        });
}
