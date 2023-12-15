mod person;
use person::Person;
fn main() {
    let person = Person::new("Mark", "O'Neill", None, None, None);
    println!("Name: {}", person.get_full_name());
    println!("Age: {:?}", person.get_age());
    println!("Email {:?}", person.get_email());
    println!("Person {:#?}", person);
}
