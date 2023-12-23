use crate::app;
use crate::app::AppId;
use crate::category;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DelegateModule: app::AppModule + category::CategoryModule {
    #[payable("*")]
    #[endpoint(delegate)]
    fn delegate_endpoint(&self, app_id: u64, category: ManagedBuffer) {
        let transfers = self.call_value().all_esdt_transfers();

        require!(!transfers.is_empty(), "no delegations provided");
        require!(self.app_ids().contains_id(app_id), "unknown app id");
        require!(self.categories(app_id).contains(&category), "unknown category");

        // TODO: check is data collection whitelisted for each transfer

        // TODO: add to delegation storage
    }

    #[view(getDelegations)]
    fn get_delegations_view(&self, app_id: AppId) {
        // TODO: implement
    }

    #[storage_mapper("delegate:delegations")]
    fn delegations(&self, app_id: AppId) -> UnorderedSetMapper<EsdtTokenPayment<Self::Api>>;
}
