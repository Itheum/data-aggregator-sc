multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait ConfigModule {
    #[endpoint(addAdmin)]
    fn add_admin_endpoint(&self, address: ManagedAddress) {
        self.require_caller_is_admin();
        self.admins().insert(address);
    }

    #[endpoint(removeAdmin)]
    fn remove_admin_endpoint(&self, address: ManagedAddress) {
        self.require_caller_is_admin();
        self.admins().swap_remove(&address);
    }

    #[endpoint(addDataCollectionWhitelist)]
    fn add_data_collection_whitelist(&self, token_identifier: TokenIdentifier) {
        self.require_caller_is_admin();
        self.data_collection_whitelist().insert(token_identifier);
    }

    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        let is_admin = self.admins().contains(&caller);
        let is_owner = self.blockchain().get_owner_address() == caller;
        require!(is_admin || is_owner, "caller must be admin");
    }

    #[view(getAdmins)]
    #[storage_mapper("config:admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("config:data_collection_whitelist")]
    fn data_collection_whitelist(&self) -> UnorderedSetMapper<TokenIdentifier>;
}
