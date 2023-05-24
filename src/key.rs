#[warn(dead_code)]
use super::config;
use bitcoin::util::key::{PrivateKey,PublicKey}; 
use bitcoin::network::constants::Network;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Address;


#[derive(Debug, Clone)]
pub struct KeyInfo{
    privatekey: PrivateKey,
    network: String
}

impl KeyInfo{

    pub fn new (conf: &config::Config) -> KeyInfo{
        KeyInfo{privatekey: PrivateKey::from_wif(&conf.wallet.private_key_for_signing).unwrap(),
                network: format!("{}", "testnet")}
    }

    pub fn get_private_key(&self) -> PrivateKey{
        self.privatekey
        //PrivateKey::from_slice(&self.privatekey, self.get_network()).unwrap()
    }

    pub fn get_public_key(&self) -> PublicKey{
        //let bitcoin_priv_key_server = PrivateKey::from_slice(&self.privatekey, self.get_network()).unwrap();
        let secp = Secp256k1::new();
        self.privatekey.public_key(&secp).clone()

    }

    pub fn get_p2pkh(&self) -> Address{
        Address::p2pkh(&self.get_public_key(), self.get_network())
    }

    //pub fn broadcast_tx(&self) -> bool{
    //    self.broadcast
    //}

    fn get_network(&self) -> Network {
        let net = match self.network.as_str(){
            "mainnet" => Network::Bitcoin,
            "testnet" => Network::Testnet,
            &_ => todo!()
        };
        net
    }

}
  
pub fn load_key_info(conf: config::Config) -> KeyInfo{
    let key_info = KeyInfo::new(&conf);
    key_info
}

