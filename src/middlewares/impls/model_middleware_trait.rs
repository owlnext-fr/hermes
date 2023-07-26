use crate::model::impls::model_trait::ModelTrait;
use anyhow::Result;

pub trait ModelMiddlewareTrait {
    fn flag_creation(&self, model: &mut impl ModelTrait) -> Result<()> {
        let now = chrono::Utc::now();

        model.set_created_date(now);
        model.set_updated_date(now);
        model.set_deleted_date(None);
        model.set_is_deleted(false);
        Ok(())
    }

    fn flag_update(&self, model: &mut impl ModelTrait) -> Result<()> {
        let now = chrono::Utc::now();

        model.set_updated_date(now);
        Ok(())
    }

    fn flag_deletion(&self, model: &mut impl ModelTrait) -> Result<()> {
        let now = chrono::Utc::now();

        model.set_updated_date(now);
        model.set_deleted_date(Some(now));
        model.set_is_deleted(true);
        Ok(())
    }


}