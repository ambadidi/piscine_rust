pub use crate::mall::floor::{self, store};
pub mod mall;

pub fn biggest_store(mall: mall::Mall) -> store::Store {
    let mut biggest: store::Store = store::Store {
        name: "MyStore".to_string(),
        square_meters: 0,
        employees: vec![],
    };

    for floor in mall.floors {
        for store in floor.stores {
            if store.square_meters > biggest.square_meters {
                biggest = store;
            }
        }
    }

    return biggest;
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<store::employee::Employee> {
    let mut highest_paid:Vec<store::employee::Employee> = Vec::new();

    for floor in mall.floors{
        for store in floor.stores{
            for employee in store.employees{
                if highest_paid.len() == 0 || employee.salary == highest_paid[0].salary{
                    highest_paid.push(employee);
                }
                else if employee.salary > highest_paid[0].salary{
                    highest_paid[0] = employee;
                }
            }
        }
    }
    return highest_paid;
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut counter: usize = 0;

    for floor in mall.floors{
        for store in floor.stores{
            counter += store.employees.len();
        }
    }

    counter += mall.guards.len();

    return  counter;
}

pub fn check_for_securities(mall: &mut mall::Mall, guards: Vec<mall::guard::Guard>) {
    let mut mall_area:u64 = 0;

    for floor in &mall.floors {
        for store in &floor.stores{
            mall_area += store.square_meters;
        }
    }

    let mut counter: u64 = 0;
    for guard in guards{
        if counter == 0 || mall_area / counter > 200 {
            mall.guards.push(guard);
            counter += 1;
        }else{
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut mall::Mall) {
    for floor in &mut mall.floors{
        for store in &mut floor.stores{
            for mut employee in &mut store.employees{
                if employee.working_hours.1 - employee.working_hours.0 > 10{
                    employee.salary += employee.salary * 0.1;
                }else{
                    employee.salary -= employee.salary * 0.1;
                }
            }
        }
    }
}
