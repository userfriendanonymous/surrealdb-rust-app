pub mod auth;
pub mod users;
pub mod posts;

use std::sync::{Arc, Mutex};
use crate::db_pool::DbPool;