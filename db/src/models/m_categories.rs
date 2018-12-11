use super::*;

#[derive(Debug, Queryable, Identifiable, AsChangeset, RestrictionForDiesel)]
#[table_name = "categories"]
#[has_many_children(Article)]
#[brothers_impl(NewCategory, brother_type = "insert")]
#[brothers_impl(UpdateCategory, brother_type = "update")]
pub struct Category {
    pub id: Uuid,
    pub super_id: Option<Uuid>,
    pub cat_name: String,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub super_id: Option<Uuid>,
    pub cat_name: String,
}

#[derive(AsChangeset)]
#[table_name = "categories"]
pub struct UpdateCategory {
    pub id: Uuid,
    pub super_id: Option<Uuid>,
    pub cat_name: String,
}
