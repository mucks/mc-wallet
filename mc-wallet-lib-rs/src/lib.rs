use bip32::Mnemonic;
use jni::{
    objects::{JObject, JString},
    strings::JNIString,
    sys::{jint, jstring},
    JNIEnv,
};
use rand_core::OsRng;

fn create_mnemonic() -> String {
    let mnemonic = Mnemonic::random(OsRng, Default::default());
    mnemonic.phrase().to_string()
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_doubleRust(
    _env: JNIEnv,
    _obj: JObject,
    x: jint,
) -> jint {
    x * 2
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createMnemonicRust<'local>(
    env: JNIEnv<'local>,
    _obj: JObject,
) -> JString<'local> {
    let mnemonic = create_mnemonic();
    env.new_string(mnemonic)
        .expect("Couldn't create java string!")
}
