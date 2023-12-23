multiversx_sc::imports!();

pub type AppId = u64;

#[multiversx_sc::module]
pub trait AppModule {
    #[endpoint(registerApp)]
    fn register_app_endpoint(&self, address: ManagedAddress) -> u64 {
        let app_id = self.app_ids().insert_new(&address);

        app_id
    }

    fn require_app_exists(&self, app_id: AppId) {
        require!(self.app_ids().contains_id(app_id), "unknown app id");
    }

    #[storage_mapper("app:ids")]
    fn app_ids(&self) -> AddressToIdMapper<Self::Api>;
}
