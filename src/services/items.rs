use sea_orm::*;
use crate::models::items::{Entity as Item, ActiveModel};
use crate::dto::items::{CreateItem, ItemResponse};

pub struct ItemService;

impl ItemService {
    pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<ItemResponse>, DbErr> {
        let items = Item::find().all(db).await?;
        Ok(items.into_iter().map(|item| ItemResponse::from(item)).collect())
    }

    pub async fn get_by_id(
        db: &DatabaseConnection,
        id: i32
    ) -> Result<Option<ItemResponse>, DbErr> {
        let item = Item::find_by_id(id).one(db).await?;
        Ok(item.map(|item| ItemResponse::from(item)))
    }

    pub async fn create(
        db: &DatabaseConnection,
        data: CreateItem,
    ) -> Result<ItemResponse, DbErr> {
        let item = ActiveModel {
            id: NotSet,
            name: Set(data.name),
            description: Set(data.description),
        };

        let result = item.insert(db).await?;
        Ok(ItemResponse::from(result))
    }

    pub async fn delete(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<u64, DbErr> {
        let result = Item::delete_by_id(id).exec(db).await?;
        Ok(result.rows_affected)
    }

}