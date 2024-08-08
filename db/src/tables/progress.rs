use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

use crate::tables::task;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "progress")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(default_value = 0)]
    pub percentage: Option<f64>,
    pub task_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "task::Entity",
        from = "Column::TaskId",
        to = "task::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Task
}

impl Related<task::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Task.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}