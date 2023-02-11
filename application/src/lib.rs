use domain::{Name, User, UserRepository};
use infrastructure::FileSyStemUserRepository;

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

pub struct Context {}
impl Context {
    pub fn create_user_usecase(&self) -> CreateUser {
        CreateUser::new(Box::new(FileSyStemUserRepository::new("./db".to_owned())))
    }
}
