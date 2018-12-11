use super::*;

#[derive(Debug, Queryable, Identifiable, Associations, RestrictionForDiesel)]
#[table_name = "tags_with_articles"]
#[belongs_to(Tag, foreign_key = "tag_id")]
#[belongs_to(Article, foreign_key = "article_id")]
#[belongs_to_tables = "tags"]
#[belongs_to_tables = "articles"]
#[brothers_impl_belongs(NewTagWithArticle, brother_type = "insert")]
#[brothers_impl_belongs(UpdateTagWithArticle, brother_type = "update")]
pub struct TagWithArticle {
    pub id: Uuid,
    pub tag_id: Uuid,
    pub article_id: Uuid,
}

#[derive(Insertable)]
#[table_name = "tags_with_articles"]
pub struct NewTagWithArticle {
    pub tag_id: Uuid,
    pub article_id: Uuid,
}

#[derive(AsChangeset)]
#[table_name = "tags_with_articles"]
pub struct UpdateTagWithArticle {
    pub id: Uuid,
    pub tag_id: Uuid,
    pub article_id: Uuid,
}
