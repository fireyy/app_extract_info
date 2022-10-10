use std::{
    env,
    path::PathBuf
};
use app_extract_info::{
    error::{ExtResult},
    get_loaders,
};

fn main() -> ExtResult<()> {
    let base_dir = env::current_dir().expect("not found path");
    test_ipa(base_dir.clone());
    test_apk(base_dir.clone());

    Ok(())
}

fn test_ipa(base_dir: PathBuf) {
    let path = base_dir.join("test.ipa");
    let result = get_loaders(&path);
    match result {
        Ok(ipa) => {
            println!("{:?}", ipa);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}

fn test_apk(base_dir: PathBuf) {
    let path_apk = base_dir.join("test.apk");
    let result = get_loaders(&path_apk);
    match result {
        Ok(apk) => {
            println!("{:?}", apk);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}