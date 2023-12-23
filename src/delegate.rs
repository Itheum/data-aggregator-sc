use crate::app;
use crate::app::AppId;

multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait DelegateModule: app::AppModule {
    #[payable("*")]
    #[endpoint(delegate)]
    fn delegate_endpoint(&self, app_id: AppId, segment: ManagedBuffer) {
        let transfers = self.call_value().all_esdt_transfers();

        require!(!transfers.is_empty(), "no delegations provided");
        require!(!segment.is_empty(), "invalid segment");
        self.require_app_exists(app_id);

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
