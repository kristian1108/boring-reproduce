use std::env;
use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::session::UserType;

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new().build().unwrap();
    let _g = client.get("https://google.com").send().await.unwrap();

    // MacOS Sonoma with Homebrew:
    // export PKCS11_MODULE='/opt/homebrew/Cellar/softhsm/2.6.1/lib/softhsm/libsofthsm2.so'
    let pkcs11 = Pkcs11::new(env::var("PKCS11_MODULE").unwrap()).unwrap();
    pkcs11.initialize(CInitializeArgs::OsThreads).unwrap();

    let slots = pkcs11.get_slots_with_initialized_token().unwrap();

    let slot = slots.into_iter().find(|&s| pkcs11.get_token_info(s).unwrap().label().trim() == "TokenZero").unwrap();
    let session = pkcs11.open_rw_session(slot).unwrap();
    let secret_pin = secrecy::Secret::new(String::from("SoftUserPin1234"));

    // Crashes with
    // Assertion failed: (ret == 0), function digest_final, file boringssl_crypto_digests.m, line 41.
    let _ = session.login(UserType::User, Some(&secret_pin));
}
