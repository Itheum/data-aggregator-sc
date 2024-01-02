use crate::config;

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub type AppId = u64;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, ManagedVecItem)]
pub struct AppInfo<M: ManagedTypeApi> {
    pub name: ManagedBuffer<M>,
    pub manager: ManagedAddress<M>,
    pub created_at: u64,
    pub data_collections: ManagedVec<M, TokenIdentifier<M>>,
}

#[multiversx_sc::module]
pub trait AppModule: config::ConfigModule {
    #[endpoint(registerApp)]
    fn register_app_endpoint(&self, name: ManagedBuffer) -> AppId {
        let caller = self.blockchain().get_caller();
        let app_id = self.next_app_id().get();
        let current_time = self.blockchain().get_block_timestamp();

        self.app_info(app_id).set(AppInfo {
            name,
            manager: caller,
            created_at: current_time,
            data_collections: ManagedVec::new(),
        });

        for collection in self.data_collection_defaults(app_id).iter() {
            self.add_data_collection(app_id, collection);
        }

        app_id
    }

    #[endpoint(unregisterApp)]
    fn unregister_app_endpoint(&self, app_id: AppId) {
        self.require_caller_is_app_manager(app_id);

        require!(self.delegations(app_id).is_empty(), "app must not have active delegations");

        self.app_info(app_id).clear();
    }

    #[endpoint(addDataCollection)]
    fn add_data_collection_endpoint(&self, app_id: AppId, collection: TokenIdentifier) {
        self.require_caller_is_app_manager(app_id);
        self.add_data_collection(app_id, collection);
    }

    #[endpoint(removeDataCollection)]
    fn remove_data_collection_endpoint(&self, app_id: AppId, collection: TokenIdentifier) {
        self.require_caller_is_app_manager(app_id);
        self.remove_data_collection(app_id, collection);
    }

    #[endpoint(addDefaultDataCollection)]
    fn add_default_data_collection_endpoint(&self, app_id: AppId, collection: TokenIdentifier) {
        self.require_caller_is_admin();
        self.data_collection_defaults(app_id).insert(collection);
    }

    #[endpoint(removeDefaultDataCollection)]
    fn remove_default_data_collection_endpoint(&self, app_id: AppId, collection: TokenIdentifier) {
        self.require_caller_is_admin();
        self.data_collection_defaults(app_id).swap_remove(&collection);
    }

    #[view(getAppInfo)]
    fn get_app_info_view(&self, app_id: AppId) -> AppInfo<Self::Api> {
        self.app_info(app_id).get()
    }

    fn add_data_collection(&self, app_id: AppId, collection: TokenIdentifier) {
        let mut app_info = self.app_info(app_id).get();
        require!(!app_info.data_collections.contains(&collection), "collection already added");

        app_info.data_collections.push(collection);
        self.app_info(app_id).set(app_info);
    }

    fn remove_data_collection(&self, app_id: AppId, collection: TokenIdentifier) {
        let mut app_info = self.app_info(app_id).get();
        let index = app_info.data_collections.iter().position(|c| *c == collection);
        require!(index.is_some(), "collection not found");

        app_info.data_collections.remove(index.unwrap());
        self.app_info(app_id).set(app_info);
    }

    fn process_app_undelegate(&self, app_id: AppId, delegator: ManagedAddress, collection: TokenIdentifier, nonce: u64) {
        let app_info = self.app_info(app_id).get();

        if !self.blockchain().is_smart_contract(&app_info.manager) {
            return;
        }

        self.app_contract(app_info.manager)
            .handle_aggregator_undelegate_endpoint(delegator, collection, nonce)
            .async_call()
            .call_and_exit_ignore_callback();
    }

    fn require_app_exists(&self, app_id: AppId) {
        require!(!self.app_info(app_id).is_empty(), "unknown app id");
    }

    fn require_caller_is_app_manager(&self, app_id: AppId) {
        let caller = self.blockchain().get_caller();
        let app_info = self.app_info(app_id).get();

        require!(app_info.manager == caller, "caller must be app manager");
    }

    #[storage_mapper("app_next_id")]
    fn next_app_id(&self) -> SingleValueMapper<AppId>;

    #[storage_mapper("app_info")]
    fn app_info(&self, app_id: AppId) -> SingleValueMapper<AppInfo<Self::Api>>;

    #[storage_mapper("data_collections_defaults")]
    fn data_collection_defaults(&self, app_id: AppId) -> UnorderedSetMapper<TokenIdentifier<Self::Api>>;

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
