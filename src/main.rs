use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let text = read_file();
    let mut modules = Vec::new();

    for line in text.unwrap().lines() {
        modules.push(line.parse::<i32>().unwrap());
    }

    let day1 = Day1 { modules };
    let total_fuel: i32 = day1.get_total_fuel();
    let day1_2_total_fuel = day1.calculate_total_fuel();

    println!("total fuel: {:?}", total_fuel);
    println!("total fuel for day 1.2: {:?}", day1_2_total_fuel);
}

fn read_file() -> Result<String, io::Error> {
    let mut file = File::open("input.txt")?;
    let mut s = String::new();

    file.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Day1 {
    modules: Vec<i32>,
}

impl Day1 {
    fn get_total_fuel(&self) -> i32 {
        self.modules
            .iter()
            .map(|module| ((module / 3) as i32 - 2))
            .sum()
    }

    fn calculate_mass_per_module(module: i32) -> i32 {
        let mass = (module / 3) as i32 - 2;

        mass
    }

    fn calculate_total_fuel(&self) -> i32 {
        let mut total_fuel: i32 = 0;

        for module in &self.modules {
            let mut fuel = *module;
            loop {
                fuel = Day1::calculate_mass_per_module(fuel);

                if fuel <= 0 {
                    break;
                }
                total_fuel = total_fuel + fuel;
            }
        }

        total_fuel
    }
}
