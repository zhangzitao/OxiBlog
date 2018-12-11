use super::*;

#[derive(Debug, Queryable, Identifiable, Associations, RestrictionForDiesel)]
#[table_name = "comments"]
#[belongs_to(Article, foreign_key = "article_id")]
#[belongs_to_tables = "articles"]
#[brothers_impl_belongs(NewComment, brother_type = "insert")]
pub struct Comment {
    pub id: Uuid,
    pub article_id: Uuid,
    pub nick_name: String,
    pub contact_address: String,
    pub content: String,
    pub create_time: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    pub article_id: Uuid,
    pub nick_name: String,
    pub contact_address: String,
    pub content: String,
    pub create_time: NaiveDateTime,
}
