use cryptoki::context::{CInitializeArgs, Pkcs11};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static HELLO: Lazy<Mutex<TestHsm>> = Lazy::new(|| Mutex::new(TestHsm::new()));

pub struct TestHsm {
    pkcs11: Pkcs11,
}

impl TestHsm {
    pub fn new() -> Self {
        let pkcs11 = Pkcs11::new(env!("PKCS11_MODULE")).unwrap();
        pkcs11.initialize(CInitializeArgs::OsThreads).unwrap();
        Self {
            pkcs11,
        }
    }
}

#[tokio::main]
async fn main() {
    let _ = HELLO.lock().await;
}
