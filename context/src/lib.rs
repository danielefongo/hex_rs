use application::CreateUser;
use infrastructure::{AuthenticateImpl, FileSystemUserRepository};

pub struct Context {}
impl Context {
    pub fn create_user_usecase(&self) -> CreateUser {
        CreateUser::new(
            Box::new(FileSystemUserRepository::new("./db".to_owned())),
            Box::new(AuthenticateImpl),
        )
    }
}
