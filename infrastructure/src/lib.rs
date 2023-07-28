use auth::get_user;
use domain::Authenticate;

pub mod file;
pub mod postgres;
mod schema;

pub struct ThreadedAuthenticate;
impl Authenticate for ThreadedAuthenticate {
    fn authenticate(&self, username: &str) -> bool {
        if let Some(authenticated_user) = get_user() {
            authenticated_user == username
        } else {
            false
        }
    }
}
