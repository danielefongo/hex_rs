use domain::{Authenticate, Name, User, UserRepository};

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

pub struct GetUser {
    users: Box<dyn UserRepository>,
    authenticate: Box<dyn Authenticate>,
}

impl GetUser {
    pub fn new(users: Box<dyn UserRepository>, authenticate: Box<dyn Authenticate>) -> Self {
        Self {
            users,
            authenticate,
        }
    }
    pub fn run(&self, name: Name) -> Result<User, Error> {
        if self.authenticate.authenticate(&name.0) {
            self.users
                .get(&name)
                .map_err(|_| Error("User not found".to_string()))
        } else {
            Err(Error("Not authenticated".to_owned()))
        }
    }
}
