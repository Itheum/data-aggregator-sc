multiversx_sc::imports!();

pub type AppId = u64;

#[multiversx_sc::module]
pub trait AppModule {
    #[endpoint(registerApp)]
    fn register_app_endpoint(&self, address: ManagedAddress) -> u64 {
        let app_id = self.app_ids().insert_new(&address);

        app_id
    }

    #[storage_mapper("app:ids")]
    fn app_ids(&self) -> AddressToIdMapper<Self::Api>;
}
