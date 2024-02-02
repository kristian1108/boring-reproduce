use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::slot::Slot;
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static HELLO: Lazy<Mutex<TestHsm>> = Lazy::new(|| Mutex::new(TestHsm::new()));

pub struct TestHsm {
    pkcs11: Pkcs11,
    slot: Slot,
}

impl TestHsm {
    pub fn new() -> Self {
        let pkcs11 = Pkcs11::new(env!("PKCS11_MODULE")).unwrap();
        pkcs11.initialize(CInitializeArgs::OsThreads).unwrap();
        let slots = pkcs11.get_slots_with_initialized_token().unwrap();
        let slot = slots.into_iter().find(|&s| pkcs11.get_token_info(s).unwrap().label().trim() == "TokenZero").unwrap();
        Self {
            pkcs11,
            slot,
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = HELLO.lock().await;
}
