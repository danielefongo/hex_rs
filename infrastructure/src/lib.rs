use std::{
    fs::{self, File},
    io::Write,
};

use auth::get_user;
use domain::{Authenticate, User, UserRepository};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FileUser {
    name: String,
    age: usize,
}
impl From<&User> for FileUser {
    fn from(user: &User) -> Self {
        FileUser {
            name: user.name.to_string(),
            age: user.age,
        }
    }
}
impl From<FileUser> for User {
    fn from(file_user: FileUser) -> Self {
        Self {
            name: domain::Name(file_user.name),
            age: file_user.age,
        }
    }
}
impl TryFrom<String> for FileUser {
    type Error = domain::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&value).map_err(|_| domain::Error("Invalid string".to_owned()))
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

    fn get(&self, name: &domain::Name) -> Result<User, domain::Error> {
        Ok(fs::read_to_string(format!("./{}/{}.txt", self.path, name))
            .map_err(|_| domain::Error("Not found".to_string()))
            .and_then(FileUser::try_from)?
            .into())
    }
}

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
