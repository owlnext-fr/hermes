use chrono::{DateTime, Utc};

pub trait ModelTrait {
    fn get_created_date(&self) -> DateTime<Utc>;
    fn get_updated_date(&self) -> DateTime<Utc>;
    fn get_deleted_date(&self) -> Option<DateTime<Utc>>;
    fn get_is_deleted(&self) -> bool;
    fn set_created_date(&mut self, created_date: DateTime<Utc>) -> &mut Self;
    fn set_deleted_date(&mut self, deleted_date: Option<DateTime<Utc>>) -> &mut Self;
    fn set_updated_date(&mut self, updated_date: DateTime<Utc>) -> &mut Self;
    fn set_is_deleted(&mut self, is_deleted: bool) -> &mut Self;
}
