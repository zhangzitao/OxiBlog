use super::*;

#[derive(Debug, Queryable, Identifiable, Associations, RestrictionForDiesel)]
#[table_name = "tags"]
#[has_many_children(TagWithArticle)]
#[brothers_impl(NewTag, brother_type = "insert")]
#[brothers_impl(UpdateTag, brother_type = "update")]
pub struct Tag {
    pub id: Uuid,
    pub tag_name: String,
}

#[derive(Insertable)]
#[table_name = "tags"]
pub struct NewTag {
    pub tag_name: String,
}

#[derive(AsChangeset)]
#[table_name = "tags"]
pub struct UpdateTag {
    pub id: Uuid,
    pub tag_name: String,
}
