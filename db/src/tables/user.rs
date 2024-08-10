use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,
}

impl Model {
    pub fn new(input_data: RegisterInput) -> ActiveModel {
        ActiveModel {
            name: Set(input_data.name),
            email: Set(input_data.email),
            password: Set(input_data.password),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}