struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}

static mut PERSON_ID: i32 = 0;

// Method
impl Person {
    fn print(&self) {
        println!("{}, {}, {}, {}", self.id, self.name, self.age, self.addr);
    }

    fn add_age(&mut self, n: i32) {
        self.age += n;
    }

    fn to_str(&self) -> String {
        let s = format!("{}: {}, {}, {}", self.id, self.name, self.age, self.addr);
        s
    }

    fn new( name: &str, age: i32, addr: &str ) -> Person {
        let id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };
        Person {
            id: id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
}

fn main() {
    let mut pa = Person::new("John", 33, "Kyoto");
    pa.print();
    pa.add_age(3);
    pa.print();
    let s = pa.to_str();
    println!("{}", s);

    let mut people = Vec::<Person>::new();
    people.push(pa);
    people.push(Person::new("Bob", 6, "California"));
    people.push(Person::new("Luk", 45, "Cheko"));
    for p in &people {
        p.print();
    }
}
