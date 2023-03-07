mod users;
mod posts;
mod utils;

use surrealdb::{Datastore, Session};
pub use users::User;
pub use posts::Post;

pub struct DbPool {
    datastore: Datastore,
    session: Session,
}

impl DbPool {
    pub async fn new() -> Self {
        let datastore = Datastore::new("memory").await.unwrap();
        let session = Session::for_db("ns", "db");

        Self {
            datastore,
            session
        }
    }
}