use anyhow::{Context, Result};
use aptos_sdk::{
    coin_client::CoinClient,
    rest_client::{Client, FaucetClient},
    types::LocalAccount,
};
use once_cell::sync::Lazy;
use std::str::FromStr;
use url::Url;

// aptos testnet url
const APTOS_TEST_NET_URL: &'static str = "https://testnet.aptoslabs.com/v1";
static TEST_NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var(APTOS_TEST_NET_URL)
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
    )
    .unwrap()
});
// aptos development network url
const APTOS_DEV_NET_URL: &'static str = "https://fullnode.devnet.aptoslabs.com/v1";
static DEV_NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var(APTOS_DEV_NET_URL)
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
    )
    .unwrap()
});
// aptos mainnet url
const APTOS_MAINNET_NET_URL: &'static str = "https://fullnode.mainnet.aptoslabs.com/v1";
static MAIN_NODE_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var(APTOS_MAINNET_NET_URL)
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://fullnode.devnet.aptoslabs.com"),
    )
    .unwrap()
});
// aptos faucet url
const APTOS_FAUCET_NET_URL: &'static str = "https://fullnode.mainnet.aptoslabs.com/v1";
static FAUCET_URL: Lazy<Url> = Lazy::new(|| {
    Url::from_str(
        std::env::var(APTOS_FAUCET_NET_URL)
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://faucet.devnet.aptoslabs.com"),
    )
    .unwrap()
});
