use jni::{
    objects::{JObject, JString},
    sys::{jint, jstring},
    JNIEnv,
};

use crate::{create_config_dir, create_mnemonic};

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createConfigDir(
    _env: JNIEnv,
    _obj: JObject,
) {
    create_config_dir().expect("could not create config dir!");
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
