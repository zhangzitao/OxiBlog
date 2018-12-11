use super::*;

#[derive(Debug, Queryable, Identifiable, Associations, RestrictionForDiesel)]
#[table_name = "articles"]
#[belongs_to(User, foreign_key = "user_id")]
#[belongs_to(Category, foreign_key = "category_id")]
#[belongs_to_tables = "users, categories"]
#[brothers_impl_belongs(NewArticle, brother_type = "insert")]
#[brothers_impl_belongs(UpdateArticle, brother_type = "update")]
#[has_many_children(TagWithArticle, Comment)]
pub struct Article {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub release_status: i16, // 0-draft 100-release 999-deleted
    pub title: String,
    pub content: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "articles"]
pub struct NewArticle {
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub release_status: i16,
    pub title: String,
    pub content: String,
}

#[derive(AsChangeset)]
#[table_name = "articles"]
pub struct UpdateArticle {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub release_status: i16,
    pub title: String,
    pub content: String,
    pub create_time: Option<NaiveDateTime>,
    pub update_time: NaiveDateTime,
}
