use domain::{Authenticate, Name, User, UserRepository};

#[derive(Debug)]
pub struct Error(String);

pub struct CreateUser {
    users: Box<dyn UserRepository>,
    authenticate: Box<dyn Authenticate>,
}

impl CreateUser {
    pub fn new(users: Box<dyn UserRepository>, authenticate: Box<dyn Authenticate>) -> Self {
        Self {
            users,
            authenticate,
        }
    }
    pub fn run(&self, name: Name) -> Result<(), Error> {
        if self.authenticate.authenticate(&name.0) {
            self.users.save(&User::new(name)).unwrap();
            Ok(())
        } else {
            Err(Error("Not authenticated".to_owned()))
        }
    }
}
