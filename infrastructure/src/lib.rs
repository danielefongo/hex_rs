use std::{fs::File, io::Write};

use auth::get_user;
use domain::{Authenticate, User, UserRepository};
use serde::Serialize;

#[derive(Serialize)]
struct FileUser {
    name: String,
}
impl From<&User> for FileUser {
    fn from(user: &User) -> Self {
        FileUser {
            name: user.name.to_string(),
        }
    }
}

pub struct FileSystemUserRepository {
    path: String,
}
impl FileSystemUserRepository {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}
impl UserRepository for FileSystemUserRepository {
    fn save(&self, user: &domain::User) -> Result<(), domain::Error> {
        let mut file = File::create(format!("./{}/{}.txt", self.path, user.name)).unwrap();
        let file_user = FileUser::from(user);
        file.write_all(serde_json::to_string(&file_user).unwrap().as_bytes())
            .unwrap();
        Ok(())
    }
}

pub struct AuthenticateImpl;
impl Authenticate for AuthenticateImpl {
    fn authenticate(&self, username: &str) -> bool {
        if let Some(authenticated_user) = get_user() {
            authenticated_user == username
        } else {
            false
        }
    }
}
