
pub use crate::error::Error;
use crate::location::Location;

#[derive(Debug)]
pub struct IllegalCharError {
    location: Location,
    character: String
}

impl Error for IllegalCharError {
    fn get(&self) -> String {
        format!("[MOLI] - sitelen ike li lon. sitelen ni li ni: \"{}\". sitelen ni li lon ni: {}", self.character, self.location)
    }
}

impl IllegalCharError {
    pub fn new(location: Location, 
        character: String) -> Self {

        Self {
            location,
            character
        }
    }
}