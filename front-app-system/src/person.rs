use chrono::{NaiveDate, Utc, Datelike};

#[allow(unused)]
#[derive(Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
    date_of_birth: Option<NaiveDate>,
    phone_number: Option<String>,
    email: Option<String>,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str, dob: Option<NaiveDate>, phone_number: Option<String>, email: Option<String>) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            date_of_birth: dob,
            phone_number,
            email,
        }
    }


}

impl Person {
    pub fn get_age(&self) -> Option<u16> {
        return if let Some(dob) = self.date_of_birth {
            let today = Utc::now().naive_utc().date();
            let age = today.year() - dob.year() - if today.month() < dob.month() || (today.month() == dob.month() && today.day() < dob.day()) { 1 } else { 0 };
            Some(age as u16)
        } else {
            None
        }
    }

    pub fn get_full_name(&self) -> String{
        let full_name = format!("{} {}", self.first_name, self.last_name);
        full_name
    }

    pub fn get_email(&self) -> Option<String> {
       self.email.clone()
    }

    pub fn get_phone_number(&self) -> Option<String>{
        self.phone_number.clone()
    }
}