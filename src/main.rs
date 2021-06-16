use forest_crypto;
use arrayref;
use ethereum_hexutil as ex;
use web3::signing;


// 0xb218c5d6af1f979ac42bc68d98a5a0d796c6ab01
fn main(){
    let sign = ex::read_hex("0x22c1c024a111a19c6e9d422ea1e390c3d6fffa63ba8648b3f763ed3a12812dc83d2b2fddf242b95506a6efc8e3237a9d42ad67d62d58fe93cb72583c1ad2322600").unwrap();
    let hash = ex::read_hex("0x73240ced7e0e5ef15925667d1deda0703d8e9a3fa384fb24239b21713bdb4eb3").unwrap();
    let h= arrayref::array_ref![hash, 0, 32];
    let s= arrayref::array_ref![sign, 0, 65];

    // the issue is with this ecrecover
    let signer = forest_crypto::signature::ecrecover(h, s);

    let k = signing::keccak256(&signer.unwrap().payload().to_raw_bytes());
    let kk = arrayref::array_ref![k, 0, 32];
    let aa = &kk[12..32];
    
    println!("{:?}", ex::to_hex(aa));
}