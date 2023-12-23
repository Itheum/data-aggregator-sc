#![no_std]

multiversx_sc::imports!();

pub mod app;
pub mod category;
pub mod config;
pub mod delegate;

#[multiversx_sc::contract]
pub trait DataAggregator: config::ConfigModule + app::AppModule + category::CategoryModule + delegate::DelegateModule {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}
}
