// Inheritance

struct Chef {
    name: String,
    age: u32,
}

impl Chef {
    fn new(name: &str, age: u32) -> Chef {
        Chef {
            name: String::from(name),
            age,
        }
    }

    fn make_chicken(&self) {
        println!("The chef makes chicken");
    }

    fn make_salad(&self) {
        println!("The chef makes salad");
    }

    fn make_special_dish(&self) {
        println!("The chef makes a special dish");
    }
}

struct ItalianChef {
    chef: Chef,
    country_of_origin: String,
}

impl ItalianChef {
    fn new(name: &str, age: u32, country_of_origin: &str) -> ItalianChef {
        ItalianChef {
            chef: Chef::new(name, age),
            country_of_origin: String::from(country_of_origin),
        }
    }

    fn make_pasta(&self) {
        println!("The chef makes pasta");
    }

    fn make_special_dish(&self) {
        println!("The chef makes chicken parm");
    }
}

fn main() {
    let my_chef = Chef::new("Gordon Ramsay", 54);
    my_chef.make_chicken();
    my_chef.make_salad();
    my_chef.make_special_dish();

    let my_italian_chef = ItalianChef::new("Massimo Bottura", 58, "Italy");
    my_italian_chef.chef.make_chicken();
    my_italian_chef.chef.make_salad();
    my_italian_chef.chef.make_special_dish();
    my_italian_chef.make_pasta();
    my_italian_chef.make_special_dish();

    println!("Chef's name: {}", my_italian_chef.chef.name);
    println!("Chef's age: {}", my_italian_chef.chef.age);
    println!("Country of origin: {}", my_italian_chef.country_of_origin);
}
