use super::*;

#[derive(Debug, Queryable, Identifiable, Associations, RestrictionForDiesel)]
#[table_name = "pages"]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to_tables = "users"]
#[brothers_impl_belongs(NewPage, brother_type = "insert")]
#[brothers_impl_belongs(UpdatePage, brother_type = "update")]
pub struct Page {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "pages"]
pub struct NewPage {
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}

#[derive(AsChangeset)]
#[table_name = "pages"]
pub struct UpdatePage {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
}
