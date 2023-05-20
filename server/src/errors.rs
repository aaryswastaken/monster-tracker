use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct NotImplementedError;

impl NotImplementedError {
    pub fn spawn() -> Box<&'static Self> {
        return Box::new(&NotImplementedError)
    }
}

impl fmt::Display for NotImplementedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not implemented Error")
    }
}

impl Error for NotImplementedError {}

#[derive(Debug, Clone)]
pub struct PriceFetchingAborted;

impl PriceFetchingAborted {
    pub fn spawn() -> Box<&'static Self> {
        return Box::new(&PriceFetchingAborted)
    }
}

impl fmt::Display for PriceFetchingAborted {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There has been an issue while fetching the price")
    }
}

impl Error for PriceFetchingAborted {}

#[derive(Debug, Clone)]
pub struct DatabaseError;

impl DatabaseError {
    pub fn spawn() -> Box<&'static Self> {
        return Box::new(&Self)
    }
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "There has been an issue whith the database")
    }
}

impl Error for DatabaseError {}
