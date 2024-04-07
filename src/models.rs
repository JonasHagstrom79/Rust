use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub id: String,
    pub born: String,
    pub burial: String,
    pub deceased: String,
    pub firstName: String,
    pub fullName: String,
    pub lastName: String,
    pub middleName: String,
    pub occupation: String,
    pub placeOfBirth: String,
}
