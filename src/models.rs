use uuid::Uuid;
use diesel::{sql_types, expression::AsExpression};

use crate::schema::{post, user};

sql_function!(
    fn get_or_create_tag(owner_id: sql_types::Uuid, tag_name: sql_types::Text) -> sql_types::Uuid;
);

#[derive(Insertable)]
#[table_name = "user"]
pub struct NewUser {
    pub email: String
}

pub struct PostTag {
    pub user_id: Uuid,
    pub tag_name: String,
}

impl AsExpression<sql_types::Uuid> for PostTag {
    type Expression = sql_types::Uuid;

    fn as_expression(self) -> Self::Expression {
        get_or_create_tag(self.user_id, self.tag_name)
    }
}

#[derive(Insertable)]
#[table_name = "post"]
pub struct NewPost {
    pub user_id: Uuid,
    #[column_name = "tag_id"]
    pub tag: PostTag,
    pub title: String,
    pub body: String,
}
