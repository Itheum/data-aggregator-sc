multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type AppId = u64;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct AppInfo<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub creator: ManagedAddress<M>,
    pub created_at: u64,
    pub contract: ManagedAddress<M>,
    pub data_collections: ManagedVec<M, TokenIdentifier<M>>,
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

        if !contract.is_zero() {
            require!(self.blockchain().is_smart_contract(&contract), "not a contract");
        }

        self.app_info(app_id).set(AppInfo {
            name,
            creator: caller,
            created_at: current_time,
            contract,
            data_collections: ManagedVec::new(),
        });

        app_id
    }

    #[endpoint(addDataCollection)]
    fn add_data_collection_endpoint(&self, app_id: AppId, collection: TokenIdentifier) {
        self.require_caller_is_app_manager(app_id);

        let mut app_info = self.app_info(app_id).get();
        require!(!app_info.data_collections.contains(&collection), "collection already added");

        app_info.data_collections.push(collection);
        self.app_info(app_id).set(app_info);
    }

    #[endpoint(removeDataCollection)]
    fn remove_data_collection_endpoint(&self, app_id: AppId, collection: TokenIdentifier) {
        self.require_caller_is_app_manager(app_id);

        let mut app_info = self.app_info(app_id).get();
        let index = app_info.data_collections.iter().position(|c| *c == collection);
        require!(index.is_some(), "collection not found");

        app_info.data_collections.remove(index.unwrap());
        self.app_info(app_id).set(app_info);
    }
    fn process_app_undelegate(&self, app_id: AppId, delegator: ManagedAddress, collection: TokenIdentifier, nonce: u64) {
        let app_info = self.app_info(app_id).get();

        if app_info.contract.is_zero() {
            return;
        }

        self.app_contract(app_info.contract)
            .handle_aggregator_undelegate_endpoint(delegator, collection, nonce)
            .async_call()
            .call_and_exit_ignore_callback();
    }

    fn require_app_exists(&self, app_id: AppId) {
        require!(self.app_ids().contains_id(app_id), "unknown app id");
    }

    fn require_caller_is_app_manager(&self, app_id: AppId) {
        let caller = self.blockchain().get_caller();
        let app_info = self.app_info(app_id).get();

        if app_info.contract.is_zero() {
            require!(app_info.creator == caller, "only creator can manage app");
        } else {
            require!(app_info.contract == caller, "only app contract can manage app");
        }
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
        fn handle_aggregator_undelegate_endpoint(&self, delegator: ManagedAddress, collection: TokenIdentifier, nonce: u64);
    }
}
