use std::fmt::Debug;

pub trait Error: Debug {
    fn get(&self) -> String;
}

pub mod illegal_char_error;