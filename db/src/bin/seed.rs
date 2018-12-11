extern crate db;
use db::models::*;
use db::schema::*;
use db::DbConnecting;

extern crate md5;

struct RootCatId;
impl RootCatId {
    fn id() -> Uuid {
        Uuid::parse_str("d1db2c21-745e-48bc-b1be-49fce210f584").unwrap()
    }
}

fn insert_root_user() -> () {
    let conn = DbConnecting::establish_connection();

    let pass = "000".to_string();
    let pass = format!("{:x}", md5::compute(pass.as_bytes()));
    let salt = "666123456".to_string();
    let pass_word = format!("{}{}", pass, salt);
    let pass_word = format!("{:x}", md5::compute(pass_word.as_bytes()));

    let root_user = NewUser {
        user_email: "123@123.com".to_string(),
        pass_word: pass_word,
        salt: salt,
        nick_name: "nick_name".to_string(),
        role_level: 9999i16,
    };

    diesel::insert_into(users::table)
        .values(&root_user)
        .get_result::<User>(&conn)
        .expect("Error saving new user");

    println!("seed inserted: User");
}

fn insert_root_cat() -> () {
    let conn = DbConnecting::establish_connection();

    let root_cat = NewCategory {
        super_id: None,
        cat_name: "Default Category".to_string(),
    };

    // diesel::insert_into(categories::table)
    //     .values(&root_cat)
    //     .get_result::<Category>(&conn)
    //     .expect("Error saving new Category");
    root_cat.insert(&conn).expect("Error saving new Category");

    let cats = categories::table.filter(categories::cat_name.eq("Default Category"));

    diesel::update(cats)
        .set(categories::id.eq(RootCatId::id()))
        .get_result::<Category>(&conn)
        .expect("Error saving new Category");
    println!("seed inserted: Category");
}

fn insert_root_article() -> () {
    let conn = DbConnecting::establish_connection();

    let user = users::table
        .filter(users::user_email.eq("123@123.com"))
        .first::<User>(&conn)
        .expect("Error loading users");

    let cat = categories::table
        .filter(categories::id.eq(RootCatId::id()))
        .first::<Category>(&conn)
        .expect("Error loading Category");

    let root_cat = NewArticle {
        user_id: user.id,
        category_id: cat.id,
        title: "Hello World !".to_string(),
        content: "This is the first blog".to_string(),
        release_status: 100i16,
    };

    diesel::insert_into(articles::table)
        .values(&root_cat)
        .get_result::<Article>(&conn)
        .expect("Error saving new Article");

    println!("seed inserted: Article");
}

fn main() {
    let conn = DbConnecting::establish_connection();

    let res_vec = users::table
        .filter(users::user_email.eq("123@123.com"))
        .limit(1)
        .load::<User>(&conn)
        .expect("Error loading users");

    if res_vec.len() < 1 {
        insert_root_user();
    } else {
        println!("Root user has been added");
        // println!("{:?}", res_vec);
    };

    let cat_vec = categories::table
        .filter(categories::id.eq(RootCatId::id()))
        .limit(1)
        .load::<Category>(&conn)
        .expect("Error loading Category");

    if cat_vec.len() < 1 {
        insert_root_cat();
    } else {
        println!("Root Category has been added");
        // println!("{:?}", cat_vec);
    };

    let res_vec = articles::table
        .filter(articles::title.eq("Hello World !"))
        .limit(1)
        .load::<Article>(&conn)
        .expect("Error loading Article");

    if res_vec.len() < 1 {
        insert_root_article();
    } else {
        println!("First Article has been added");
        // println!("{:?}", res_vec);
    };

    // without foreign key restrict, delete could be done
    // diesel::delete(categories::table.filter(categories::cat_root.eq(true)))
    //     .execute(&conn).expect("delete wrong");
    // diesel::delete(articles::table.filter(articles::title.eq("Hello World !")))
    //     .execute(&conn).expect("delete wrong");
    let cat = categories::table
        .filter(categories::id.eq(RootCatId::id()))
        .first::<Category>(&conn)
        .expect("");
    println!(
        "[test has any child] {:?}",
        cat.has_any_child(&conn) == Ok(true)
    );
    println!(
        "[test restrict delete] {:?}",
        cat.restrict_delete(&conn) == Ok(0)
    );

    // restrict insert article
    let user = users::table
        .filter(users::user_email.eq("123@123.com"))
        .first::<User>(&conn)
        .expect("Error loading users");

    let cat = categories::table
        .filter(categories::id.eq(RootCatId::id()))
        .first::<Category>(&conn)
        .expect("Error loading Category");

    let article = NewArticle {
        user_id: user.id,
        category_id: cat.id,
        title: "Second Hello World !".to_string(),
        content: "This is the second blog".to_string(),
        release_status: 100i16,
    };
    println!(
        "[test has all parent] {:?}",
        article.has_all_parents(&conn) == Ok(true)
    );
    println!(
        "[test restrict insert] {:?}",
        article.restrict_insert(&conn) == Ok(1)
    );

    let old_article = articles::table
        .filter(articles::title.eq("Second Hello World !"))
        .first::<Article>(&conn)
        .expect("");
    let article = UpdateArticle {
        id: old_article.id,
        user_id: user.id,
        category_id: cat.id,
        title: "Hello World !".to_string(),
        content: "This is the 2 blog".to_string(),
        release_status: 100i16,
        create_time: None,
        update_time: NaiveDateTime::from_timestamp(chrono::Local::now().timestamp(), 6),
    };
    println!(
        "[test restrict update] {:?}",
        article.restrict_update(&conn) == Ok(1)
    );

    let tag = NewTag {
        tag_name: "yo!".to_string(),
    };
    println!("[test insert] {:?}", tag.insert(&conn) == Ok(1));

    let old_tag = tags::table
        .filter(tags::tag_name.eq("yo!"))
        .first::<Tag>(&conn)
        .expect("");

    let tag_with_article = NewTagWithArticle {
        tag_id: old_tag.id,
        article_id: old_article.id,
    };
    println!(
        "[test insert] {:?}",
        tag_with_article.restrict_insert(&conn) == Ok(1)
    );

    println!(
        "[test restrict delete] {:?}",
        old_article.restrict_delete(&conn) == Ok(0)
    );

    let old_tag_with_article = tags_with_articles::table
        .filter(tags_with_articles::tag_id.eq(old_tag.id))
        .first::<TagWithArticle>(&conn)
        .expect("");

    println!(
        "[test delete] {:?}",
        old_tag_with_article.delete(&conn) == Ok(1)
    );

    println!(
        "[test restrict delete] {:?}",
        old_article.restrict_delete(&conn) == Ok(1)
    );

    let update_tag = UpdateTag {
        id: old_tag.id,
        tag_name: "yo ho!".to_string(),
    };
    println!("[test update] {:?}", update_tag.update(&conn) == Ok(1));
}
