use application::{CreateUser, GetUser};
use infrastructure::ThreadedAuthenticate;

use infrastructure::postgres::{get_pool, PgPool, PgUserRepository};

pub struct Context {
    pool: PgPool,
}
impl Context {
    pub fn new() -> Self {
        Self { pool: get_pool() }
    }
    pub fn create_user_usecase(&self) -> CreateUser {
        CreateUser::new(Box::new(PgUserRepository::new(self.pool.clone())))
    }
    pub fn get_user_usecase(&self) -> GetUser {
        GetUser::new(
            Box::new(PgUserRepository::new(self.pool.clone())),
            Box::new(ThreadedAuthenticate),
        )
    }
}
