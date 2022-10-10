use std::{
    env,
    fs,
    io::{BufReader, Read},
};
use app_extract_info::{
    error::{Error, ExtResult},
    get_loaders,
};

fn main() -> ExtResult<()> {
    let base_dir = env::current_dir().expect("not found path");
    let path = base_dir.join("test.ipa");
    // let mut file = fs::File::open(&path)?;
    let result = get_loaders(&path);
    match result {
        Ok(ipa) => {
            println!("{:?}", ipa);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
    Ok(())
}