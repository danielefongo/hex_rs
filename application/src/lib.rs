use domain::{Name, User, UserRepository};

#[derive(Debug)]
pub struct Error(String);

pub struct CreateUser {
    users: Box<dyn UserRepository>,
}

impl CreateUser {
    pub fn new(users: Box<dyn UserRepository>) -> Self {
        Self { users }
    }
    pub fn run(&self, name: Name) -> Result<(), Error> {
        self.users.save(&User::new(name)).unwrap();
        Ok(())
    }
}
