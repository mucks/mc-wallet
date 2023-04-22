use jni::{
    objects::{JObject, JString},
    sys::jboolean,
    sys::jint,
    JNIEnv,
};

use crate::{
    create_and_save_seed, create_config_dir, create_mnemonic, create_wallet, get_seed,
    is_wallet_created,
};

fn jstring_to_string(env: &mut JNIEnv, jstring: JString) -> String {
    env.get_string(&jstring)
        .expect("Couldn't get java string!")
        .to_str()
        .expect("Couldn't convert java string to rust string!")
        .to_owned()
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_isWalletCreated(
    _env: JNIEnv,
    _obj: JObject,
) -> jboolean {
    let is_wallet_created = is_wallet_created();
    jboolean::from(is_wallet_created)
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createWallet<'local>(
    mut env: JNIEnv<'local>,
    _obj: JObject<'local>,
    wallet_password: JString,
) -> JString<'local> {
    let wallet_password = jstring_to_string(&mut env, wallet_password);
    let mnemonic = create_wallet(wallet_password)
        .map_err(|err| anyhow::anyhow!("could not create wallet: {}", err))
        .unwrap();
    env.new_string(mnemonic)
        .expect("Couldn't create java string!")
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createAndSaveSeed(
    mut env: JNIEnv,
    _obj: JObject,
    mnemonic: JString,
    // the password is used as an extra layer of security when creating the seed from the mnemonic
    seed_password: JString,
    encryption_password: JString,
) {
    let mnemonic = jstring_to_string(&mut env, mnemonic);
    let seed_password = jstring_to_string(&mut env, seed_password);
    let encryption_password = jstring_to_string(&mut env, encryption_password);

    create_and_save_seed(&mnemonic, &seed_password, &encryption_password)
        .map_err(|err| anyhow::anyhow!("could not create and save seed: {}", err))
        .unwrap();
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_getSeedHex<'local>(
    mut env: JNIEnv<'local>,
    _obj: JObject<'local>,
    encryption_password: JString,
) -> JString<'local> {
    let encryption_password = jstring_to_string(&mut env, encryption_password);
    let seed = get_seed(&encryption_password)
        .map_err(|err| anyhow::anyhow!("could not get seed: {}", err))
        .unwrap();
    let seed = hex::encode(seed.as_bytes());
    env.new_string(seed).expect("Couldn't create java string!")
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createConfigDir(
    _env: JNIEnv,
    _obj: JObject,
) {
    create_config_dir().expect("could not create config dir!");
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createMnemonic<'local>(
    env: JNIEnv<'local>,
    _obj: JObject,
) -> JString<'local> {
    let mnemonic = create_mnemonic();
    env.new_string(mnemonic)
        .expect("Couldn't create java string!")
}
