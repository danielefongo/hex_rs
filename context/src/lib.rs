use application::{CreateUser, GetUser};
use infrastructure::{FileSystemUserRepository, ThreadedAuthenticate};

pub struct Context {}
impl Context {
    pub fn create_user_usecase(&self) -> CreateUser {
        CreateUser::new(Box::new(FileSystemUserRepository::new("./db".to_owned())))
    }
    pub fn get_user_usecase(&self) -> GetUser {
        GetUser::new(
            Box::new(FileSystemUserRepository::new("./db".to_owned())),
            Box::new(ThreadedAuthenticate),
        )
    }
}
