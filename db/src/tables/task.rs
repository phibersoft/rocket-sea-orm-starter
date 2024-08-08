use sea_orm::ActiveValue::Set;
use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

use crate::tables::progress;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "tasks")]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Integer")]
    pub id: i32,
    pub title: String,
}

impl Model {
    pub fn new(input_data: InputData) -> ActiveModel {
        ActiveModel {
            title: Set(input_data.title),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct InputData {
    pub title: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "progress::Entity")]
    Progress
}

impl Related<progress::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Progress.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}