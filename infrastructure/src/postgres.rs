use diesel::pg::PgConnection;
use diesel::r2d2::{
    ConnectionManager, Pool as GenericPool, PooledConnection as GenericPooledConnection,
};
use diesel::{query_dsl::methods::FindDsl, RunQueryDsl};
use diesel::{Insertable, Queryable};
use domain::UserRepository;
use dotenv::dotenv;
use std::env;

pub type PgPool = GenericPool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = GenericPooledConnection<ConnectionManager<PgConnection>>;

use crate::schema::users;

fn database_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn get_pool() -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    PgPool::builder()
        .max_size(2)
        .build(manager)
        .expect("Could not initialize postgresql connection pool")
}

pub fn get_conn(pool: &PgPool) -> Result<PgPooledConnection, domain::Error> {
    pool.get()
        .map_err(|_| domain::Error("Unable to get a new connection".to_string()))
}

#[derive(Debug, Insertable, PartialEq, Eq, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    name: String,
    age: i32,
}
impl From<User> for domain::User {
    fn from(user: User) -> Self {
        Self {
            name: domain::Name(user.name),
            age: user.age as usize,
        }
    }
}
impl From<domain::User> for User {
    fn from(user: domain::User) -> Self {
        Self {
            name: user.name.to_string(),
            age: user.age as i32,
        }
    }
}

pub struct PgUserRepository {
    pool: PgPool,
}
impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
impl UserRepository for PgUserRepository {
    fn save(&self, user: &domain::User) -> Result<(), domain::Error> {
        let conn = &mut get_conn(&self.pool)?;

        diesel::insert_into(users::table)
            .values(User::from(user.clone()))
            .execute(conn)
            .map(|_| ())
            .map_err(|_| domain::Error("Failed to save user".to_string()))
    }

    fn get(&self, name: &domain::Name) -> Result<domain::User, domain::Error> {
        let conn = &mut get_conn(&self.pool)?;

        let user: User = users::table
            .find(name.to_string())
            .first(conn)
            .map_err(|_| domain::Error("Failed to get user".to_string()))?;

        Ok(user.into())
    }
}
