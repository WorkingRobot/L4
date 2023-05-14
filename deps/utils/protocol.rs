use std::{env::current_exe, io};
use winreg::{enums, transaction::Transaction, RegKey};

// `register_protocol("L4 Integration - Epic Games", "com.epicgames.fortnite", "proto-epicgames")
pub fn register_protocol(name: &str, schema: &str, subcommand: &str) -> io::Result<()> {
    let t = Transaction::new()?;
    let hkcu = RegKey::predef(enums::HKEY_CURRENT_USER);
    let (key, _disp) = hkcu.create_subkey_transacted_with_flags(
        format!("SOFTWARE\\Classes\\{schema}"),
        &t,
        enums::KEY_WRITE,
    )?;

    key.set_value("", &name)?;
    key.set_value("URL Protocol", &"")?;

    key.create_subkey_transacted_with_flags("shell\\open\\command", &t, enums::KEY_WRITE)?;

    key.set_value(
        "",
        &format!("\"{}\" \"{subcommand}\" \"%1\"", current_exe()?.display()),
    )?;

    t.commit()
}
