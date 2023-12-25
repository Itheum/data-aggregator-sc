multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type AppId = u64;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct AppInfo<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub creator: ManagedAddress<M>,
    pub created_at: u64,
    pub contract: ManagedAddress<M>,
}

#[multiversx_sc::module]
pub trait AppModule {
    #[endpoint(registerApp)]
    fn register_app_endpoint(&self, name: ManagedBuffer, contract: OptionalValue<ManagedAddress>) -> AppId {
        let caller = self.blockchain().get_caller();
        let contract = contract.into_option().unwrap_or_default();
        let app_address = if contract.is_zero() { &caller } else { &contract };
        let app_id = self.app_ids().insert_new(&app_address);
        let current_time = self.blockchain().get_block_timestamp();

        self.app_info(app_id).set(AppInfo {
            name,
            creator: caller,
            created_at: current_time,
            contract,
        });

        app_id
    }

    fn require_app_exists(&self, app_id: AppId) {
        require!(self.app_ids().contains_id(app_id), "unknown app id");
    }

    #[storage_mapper("app:ids")]
    fn app_ids(&self) -> AddressToIdMapper<Self::Api>;

    #[storage_mapper("app:info")]
    fn app_info(&self, app_id: AppId) -> SingleValueMapper<AppInfo<Self::Api>>;

    #[proxy]
    fn app_contract(&self, to: ManagedAddress) -> app_contract_proxy::Proxy<Self::Api>;
}

mod app_contract_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
    pub trait DataAggregatorContractProxy {
        #[endpoint(handleAggregatorUndelegate)]
        fn handle_aggregator_undelegate_endpoint(&self);
    }
}
