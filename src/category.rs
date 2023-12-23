multiversx_sc::imports!();

use crate::app::AppId;

pub type CategoryName<M> = ManagedBuffer<M>;

#[multiversx_sc::module]
pub trait CategoryModule {
    #[endpoint(addCategory)]
    fn add_category(&self, app_id: AppId, category: CategoryName<Self::Api>) {
        require!(!self.categories(app_id).contains(&category), "category already exists");

        self.categories(app_id).insert(category);
    }

    #[storage_mapper("category:categories")]
    fn categories(&self, app_id: AppId) -> UnorderedSetMapper<ManagedBuffer>;
}
