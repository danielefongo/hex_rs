use std::fmt::Display;

#[derive(Debug)]
pub struct Error(pub String);

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Name(pub String);
impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub name: Name,
}
impl User {
    pub fn new(name: Name) -> Self {
        Self { name }
    }
}

pub trait UserRepository {
    fn save(&self, user: &User) -> Result<(), Error>;
    fn get(&self, name: &Name) -> Result<User, Error>;
}

pub trait Authenticate {
    fn authenticate(&self, username: &str) -> bool;
}
