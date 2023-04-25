use jni::{
    objects::{JObject, JObjectArray, JString, JValue},
    sys::{jboolean, jint},
    JNIEnv,
};

use anyhow::Result;

use crate::{
    account::create_account,
    coin_type::CoinType,
    state::{get_seed_from_state, is_wallet_unlocked},
    storage::FileStorage,
    wallet::{create_wallet, is_wallet_created, test_create_wallet},
};

fn jstring_to_string(env: &mut JNIEnv, jstring: JString) -> String {
    env.get_string(&jstring)
        .expect("Couldn't get java string!")
        .to_str()
        .expect("Couldn't convert java string to rust string!")
        .to_owned()
}

fn create_wallet_rust(test: bool, password: &str) -> Result<String> {
    if test {
        test_create_wallet(password)
    } else {
        create_wallet(password)
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createAccountRust(
    mut env: JNIEnv,
    _obj: JObject,
    coin_type: jint,
) {
    if let Err(err) = java_create_account(coin_type) {
        env.throw(format!("Could not create account: {err}"))
            .unwrap();
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_getAccountsRust<'local>(
    mut env: JNIEnv<'local>,
    _obj: JObject<'local>,
) -> JObject<'local> {
    let address = env.new_string("hello").unwrap();
    let addr_value = JValue::from(&address);
    let class = env
        .find_class("dev/mucks/mc_wallet_lib/Account")
        .expect("failed to find Account class");

    env.new_object(class, "(Ljava/lang/String;)V", &[addr_value])
        .unwrap()
}

fn java_create_account(coin_type: i32) -> Result<()> {
    let coin_type = CoinType::try_from(coin_type)?;
    let mut s = FileStorage::get_from_file()?;
    let seed = get_seed_from_state()?;

    create_account(&seed, coin_type, &mut s)
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_createWalletRust<'local>(
    mut env: JNIEnv<'local>,
    _obj: JObject<'local>,
    password: JString<'local>,
    test: jboolean,
) -> JString<'local> {
    let password = jstring_to_string(&mut env, password);
    let res = create_wallet_rust(test != 0, &password);
    match res {
        Ok(mnemonic) => env.new_string(mnemonic).unwrap(),
        Err(err) => {
            env.throw(format!("Could not create wallet {err}")).unwrap();
            env.new_string("ERROR").unwrap()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_isWalletCreatedRust(
    _env: JNIEnv,
    _obj: JObject,
) -> jboolean {
    is_wallet_created() as u8
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_unlockWalletRust(
    mut env: JNIEnv,
    _obj: JObject,
    password: JString,
) {
    let wallet_password = jstring_to_string(&mut env, password);
    match crate::wallet::unlock_wallet(&wallet_password) {
        Ok(_) => {}
        Err(err) => {
            env.throw(format!("Could not unlock wallet {err}")).unwrap();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_dev_mucks_mc_1wallet_1lib_McWalletLib_isWalletUnlockedRust(
    _env: JNIEnv,
    _obj: JObject,
) -> jboolean {
    is_wallet_unlocked() as u8
}
