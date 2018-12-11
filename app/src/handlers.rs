pub mod h_articles;
pub mod h_home;
pub mod h_search;
pub mod h_users;

pub use self::h_articles::*;
pub use self::h_home::*;
pub use self::h_search::*;
pub use self::h_users::*;

use actix_web::actix::{Handler, Message};
use actix_web::{error, Error, Result};
use serde_derive::Deserialize;
use short_for_actix::GenMessage;

use crate::routes::DbExecutor;

use db::models::*;
use db::schema::*;
use db::DbConn;
use view::views::*;
