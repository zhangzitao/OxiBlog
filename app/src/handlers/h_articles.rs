use super::*;

#[derive(GenMessage)]
pub struct ArticlesIndex {}

impl Handler<ArticlesIndex> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, _msg: ArticlesIndex, _: &mut Self::Context) -> Self::Result {
        let conn: &DbConn = &self.0.get().unwrap();
        let items = articles::table
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let articles = ViewArticleIndex { articles: items };
        Ok(articles.render().unwrap())
    }
}

#[derive(GenMessage)]
pub struct ShowArticle {
    pub id: String,
}

impl Handler<ShowArticle> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: ShowArticle, _: &mut Self::Context) -> Self::Result {
        let conn: &DbConn = &self.0.get().unwrap();
        let id = Uuid::parse_str(&msg.id).map_err(error::ErrorBadRequest)?;
        let mut items = articles::table
            .filter(articles::id.eq(id))
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let item = items.pop().unwrap();
        let option = comrak::ComrakOptions {
            ext_strikethrough: true,
            ext_table: true,
            ext_tasklist: true,
            ext_superscript: true,
            ..comrak::ComrakOptions::default()
        };
        let view_article = ViewArticle {
            category_id: item.category_id,
            title: item.title,
            content: ammonia::clean(&comrak::markdown_to_html(&item.content, &option)),
            create_time: item.create_time,
            update_time: item.update_time,
        };
        Ok(view_article.render().unwrap())
    }
}

#[derive(GenMessage, Deserialize, Clone)]
pub struct NewArticleForm {
    pub user_id: String,
    pub category_id: String,
    pub release_status: i16,
    pub title: String,
    pub content: String,
}

impl Handler<NewArticleForm> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: NewArticleForm, _: &mut Self::Context) -> Self::Result {
        if msg.user_id == "".to_string() {
            return Ok("no user cookie".to_string());
        };
        let conn: &DbConn = &self.0.get().unwrap();
        let new_article = NewArticle {
            user_id: Uuid::parse_str(&msg.user_id).map_err(error::ErrorBadRequest)?,
            category_id: Uuid::parse_str(&msg.category_id).map_err(error::ErrorBadRequest)?,
            title: msg.title,
            content: msg.content,
            release_status: msg.release_status,
        };
        new_article
            .restrict_insert(&conn)
            .map_err(error::ErrorInternalServerError)?;
        Ok("".to_string())
    }
}

#[derive(GenMessage)]
pub struct EditArticle {
    pub id: String,
    pub user_id: String,
}

impl Handler<EditArticle> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: EditArticle, _: &mut Self::Context) -> Self::Result {
        if msg.user_id == "".to_string() {
            return Ok("no user cookie".to_string());
        };
        let conn: &DbConn = &self.0.get().unwrap();
        let id = Uuid::parse_str(&msg.id).map_err(error::ErrorBadRequest)?;
        let mut items = articles::table
            .filter(articles::id.eq(id))
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;

        let item = items.pop().unwrap();

        let article_for_edit = ViewArticleEdit {
            id: item.id,
            user_id: item.user_id,
            category_id: item.category_id,
            title: item.title,
            content: item.content,
            release_status: item.release_status,
            create_time: Some(item.create_time),
            update_time: item.update_time,
        };
        let ret = if item.user_id.to_string() == msg.user_id {
            Ok(article_for_edit.render().unwrap())
        } else {
            Ok("wrong user".to_string())
        };

        ret
    }
}

#[derive(GenMessage, Deserialize, Clone)]
pub struct UpdateArticleForm {
    pub id: String,
    pub user_id: String,
    pub category_id: String,
    pub release_status: i16,
    pub title: String,
    pub content: String,
}

impl Handler<UpdateArticleForm> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: UpdateArticleForm, _: &mut Self::Context) -> Self::Result {
        if msg.user_id == "".to_string() {
            return Err(error::ErrorBadRequest("no user cookie"));
        };
        let conn: &DbConn = &self.0.get().unwrap();

        let id = Uuid::parse_str(&msg.id).map_err(error::ErrorBadRequest)?;
        let mut items = articles::table
            .filter(articles::id.eq(id))
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let item = items.pop().unwrap();

        if msg.user_id != item.user_id.to_string() {
            return Err(error::ErrorBadRequest("wrong user"));
        };

        let update = UpdateArticle {
            id: item.id,
            user_id: item.user_id,
            category_id: Uuid::parse_str(&msg.category_id).map_err(error::ErrorBadRequest)?,
            title: msg.title,
            content: msg.content,
            release_status: msg.release_status,
            create_time: None,
            update_time: NaiveDateTime::from_timestamp(Local::now().timestamp(), 6),
        };
        update
            .restrict_update(&conn)
            .map_err(error::ErrorInternalServerError)?;
        Ok("".to_string())
    }
}

#[derive(GenMessage)]
pub struct DeleteArticle {
    pub id: String,
    pub user_id: String,
}

impl Handler<DeleteArticle> for DbExecutor {
    type Result = Result<String, Error>;

    fn handle(&mut self, msg: DeleteArticle, _: &mut Self::Context) -> Self::Result {
        if msg.user_id == "".to_string() {
            return Err(error::ErrorBadRequest("no user cookie"));
        };
        let conn: &DbConn = &self.0.get().unwrap();

        let id = Uuid::parse_str(&msg.id).map_err(error::ErrorBadRequest)?;
        let mut items = articles::table
            .filter(articles::id.eq(id))
            .load::<Article>(conn)
            .map_err(error::ErrorInternalServerError)?;
        let item = items.pop().unwrap();

        if msg.user_id != item.user_id.to_string() {
            return Err(error::ErrorBadRequest("wrong user"));
        };

        item.restrict_delete(&conn)
            .map_err(error::ErrorInternalServerError)?;
        Ok("".to_string())
    }
}
