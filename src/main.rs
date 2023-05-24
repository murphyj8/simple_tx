use sv::messages::{OutPoint,Tx, TxIn, TxOut, Payload};
use sv::transaction::p2pkh::{create_lock_script, create_unlock_script};
use sv::script::Script;
use sv::util::Hash256;
use sv::util::hash160;
use sv::address::addr_decode;
use sv::transaction::sighash::{sighash, SigHashCache, SIGHASH_ALL, SIGHASH_FORKID};
use sv::transaction::generate_signature;
use sv::network::Network;
//use std::io::Cursor;
use clap::Parser;
mod utils;
mod config; 
mod key; 
//use utils::decode_hexstr;

use sv::util::Serializable;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "../data/config.toml")]
    toml_config_file: String,

    #[arg(short, long, default_value = "../web.toml")]
    web_config_file: String,
}

/// Convert a transaction into a hexstring
pub fn tx_as_hexstr(tx: &Tx) -> String {
    let mut b = Vec::with_capacity(tx.size());
    tx.write(&mut b).unwrap();
    hex::encode(&b)
}

fn main(){
    let args = Args::parse();
    println!("Path to config file {}", args.toml_config_file);
    let conf_file = args.toml_config_file; 

    let contents =  match fs::read_to_string(conf_file.clone()){
        Ok(c) => c,
        Err(error) => {
            panic!("Could not read file {} because of error {}", conf_file, error);
        }
    };

    let config: config::Config = match toml::from_str(&contents){
        Ok(config) => config,
        Err(error) => {
            panic!("Could not read config because of error {}", error);
        }
    };

    println!("{:?}", config);

    let vins: Vec<TxIn> = vec![TxIn {
        prev_output: OutPoint {
            hash: Hash256::decode(&config.transactioninputs.tx_hash).unwrap(),
            index: config.transactioninputs.tx_pos,
        },
        unlock_script: Script::new(),
        sequence: 0xffffffff,
    }]; 

    let hash_addr: sv::util::Hash160 = addr_decode(&config.transactionoutputs.public_key, Network::Testnet).unwrap().0; 
    let lock_script_mess = create_lock_script(&hash_addr);
    let vouts: Vec<TxOut> = vec![
        TxOut {
            satoshis: config.transactionoutputs.amount as i64,
            lock_script: lock_script_mess.clone()
        }];

    let mut tx = Tx {
        version: 1,
        inputs: vins,
        outputs: vouts,
        lock_time: 0,
    };

    // Sign transaction
    let priv_key = key::KeyInfo::new(&config); 
    let mut cache = SigHashCache::new();
    let sighash_type = SIGHASH_ALL | SIGHASH_FORKID;

    let script_pub_key_funding_bytes = hex::decode(&config.transactioninputs.scriptpubkey).unwrap(); 
    let mut script_pub_key_funding = Script::new(); 
    script_pub_key_funding.append_slice(&script_pub_key_funding_bytes);

    let sighash = sighash(
        &tx,
        0,
        &script_pub_key_funding.0,
        config.transactioninputs.amount as i64,
        sighash_type,
        &mut cache,
    )
    .unwrap();

    let signature = generate_signature(
        &priv_key.get_private_key().to_bytes().try_into().unwrap(),
        &sighash,
        sighash_type,
    )
    .unwrap();

    tx.inputs[0].unlock_script =
            create_unlock_script(&signature, &priv_key.get_public_key().to_bytes().try_into().unwrap());
   
    dbg!(tx.clone());
    let tx_hex = tx_as_hexstr(&tx);
    println!("{}", tx_hex);

}
/* 
fn main() {
    println!("Hello, world!");

    let tx_string = "0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600";
    //let my_bytes: Vec<u8> = tx_string.as_bytes().to_vec();

    let bytes = decode_hexstr(tx_string).unwrap();

    //let bytes = match decode_hexstr(&tx_string) {
    //    Ok(b) => b,
    //    Err(_) => {
    //        println!("Failed to decode hexstr");
    //
    //    }
    //};

    //let version = tx_bytes.read_i32::<LittleEndian>()?;
    //let version = version as u32;

    let tx_var : Tx = Tx::read(&mut Cursor::new(&bytes)).unwrap();

    println!("{:?}", tx_var);
}
*/
