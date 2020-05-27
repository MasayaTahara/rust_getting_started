struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

// function with struct parameter
fn print_person( pa: &Person ) {
    println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);
}

// function with mutable struct parameter
fn plus_age( pa: &mut Person ) {
    pa.age += 1;
}

// function with struct return
fn new_person( id: i32, name: &str ) -> Person {
    let pa = Person {
        id: id,
        name: name.to_string(),
        age: -1,
        addr: String::from("Unknown"),
    };
    pa
}

fn main() {
    let mut pa = Person {
    // let pa = Person {
        id: 100,
        name: String::from("John"),
        age: 20,
        addr: String::from("New york"),
    };
    println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);
    pa.age += 1;
    println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);
    pa.addr = String::from("Osaka");
    println!("{}, {}, {}, {}", pa.id, pa.name, pa.age, pa.addr);

    plus_age(&mut pa);
    print_person(&pa);

    let pa2 = new_person(9999, "Bob");
    print_person(&pa2);

    let mut people = vec![pa, pa2];
    people.push(new_person(0001, "Erick"));
    people.push(new_person(0808, "Paul"));
    for p in &people {
        print_person(p);
    }
}
