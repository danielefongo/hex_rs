use std::future::Future;
use tokio::task::futures::TaskLocalFuture;

tokio::task_local! {
    static USER: Option<String>;
}

pub fn with_user<F>(user: Option<String>, f: F) -> TaskLocalFuture<Option<String>, F>
where
    F: Future,
{
    USER.scope(user, f)
}

pub fn get_user() -> Option<String> {
    USER.with(|u| u.clone())
}
