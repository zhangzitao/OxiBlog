pub mod v_articles;
pub mod v_home;
pub mod v_search;
pub mod v_users;

pub use self::v_articles::*;
pub use self::v_home::*;
pub use self::v_search::*;
pub use self::v_users::*;

pub use askama::Template;
use db::models::*;
