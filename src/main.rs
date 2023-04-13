use std::str::FromStr;

use hmac::{Hmac, Mac};
use sha2::Sha256;

fn main() {
    use tss_esapi::{Context, TctiNameConf};

    //connection to the TPM
    let tcti_name = "device:/dev/tpmrm0";

    let tcti = TctiNameConf::from_str(tcti_name).unwrap();
    let mut context = Context::new(tcti).unwrap();

    //generation of a subkey from a master key using HKDF
    type HmacSha256 = Hmac<Sha256>;
    let master_key = [0x01; 32];
    let salt = [0x02; 16];

    let mut hmac = HmacSha256::new_from_slice(&master_key).unwrap();

    hmac.update(&salt);

    let prk = hmac.finalize().into_bytes();

    let info = [0x03; 8];
    let subkey_len: i32 = 16;

    let mut hmac = Hmac::<Sha256>::new_from_slice(&prk).unwrap();

    hmac.update(&info);
    hmac.update(&[1u8]);
    hmac.update(&subkey_len.to_be_bytes());
    let subkey = hmac.finalize().into_bytes();
}
