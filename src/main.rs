use forest_crypto;
use arrayref;
use ethereum_hexutil as ex;

fn main(){
    // binance block https://bscscan.com/block/8312800
    // the extra data signature
    let extra_data = ex::read_hex("0x22c1c024a111a19c6e9d422ea1e390c3d6fffa63ba8648b3f763ed3a12812dc83d2b2fddf242b95506a6efc8e3237a9d42ad67d62d58fe93cb72583c1ad2322600").unwrap();
    let sign = arrayref::array_ref!(extra_data[extra_data.len()-65..extra_data.len()], 0, 65);
    // block hash
    let a = ex::read_hex("0x73240ced7e0e5ef15925667d1deda0703d8e9a3fa384fb24239b21713bdb4eb3").unwrap();
    let h= arrayref::array_ref![a, 0, 32];
    // recover pub key
    let signer = forest_crypto::signature::ecrecover(h, sign);
    let r = hex::encode(&signer.unwrap().payload().to_raw_bytes());
    // print
    println!("{:?}", r);
}