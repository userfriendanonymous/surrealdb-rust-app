mod users;

use surrealdb::{Datastore, Session};

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