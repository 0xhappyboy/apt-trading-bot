use anyhow::Ok;
use aptos_sdk::{
    coin_client::CoinClient,
    rest_client::{Client, FaucetClient},
    types::LocalAccount,
};

// net type
pub enum NET_TYPE {
    TEST,
    DEV,
    MAIN,
}

// aptos client
struct AptClient<'a> {
    rest: Option<Client>,
    faucet: Option<FaucetClient>,
    coin: Option<CoinClient<'a>>,
}

impl AptClient {
    pub fn New(net_type: NET_TYPE) -> Result<Client, String> {
        match net_type {
            NET_TYPE::MAIN => {
                //DOTO
                return Ok(None, _);
            }
            NET_TYPE::TEST => {
                //DOTO
                return Ok(None, _);
            }
            NET_TYPE::DEV => {
                //DOTO
                return Ok(None, _);
            }
            _ => {
                println!("error");
                return Err(());
            }
        }
    }
}
