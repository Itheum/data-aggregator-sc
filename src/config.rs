use crate::{app::AppId, delegate::Delegation};

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

    fn require_caller_is_admin(&self) {
        let caller = self.blockchain().get_caller();
        let is_admin = self.admins().contains(&caller);
        let is_owner = self.blockchain().get_owner_address() == caller;
        require!(is_admin || is_owner, "caller must be admin");
    }

    #[storage_mapper("users")]
    fn users(&self) -> UserMapper;

    #[view(getAdmins)]
    #[storage_mapper("admins")]
    fn admins(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[storage_mapper("delegations")]
    fn delegations(&self, app_id: AppId) -> MapMapper<usize, ManagedVec<Delegation<Self::Api>>>;
}
