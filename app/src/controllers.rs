#[macro_use]
pub mod share;

pub mod articles;
pub mod home;
pub mod search;
pub mod users;

use time::Duration;

use crate::jwt;
use actix_web::middleware::identity::*;
use actix_web::*;
use futures::Future;
use serde_derive::{Deserialize, Serialize};

pub use self::share::*;

use crate::handlers::*;
use crate::routes::AppState;

use db::models::*;
use view::views::*;
