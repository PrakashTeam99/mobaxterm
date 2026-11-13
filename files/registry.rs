//! Windows registry read helpers for Mobaxterm.

pub const APP_KEY: &str = "Software\\Mobaxterm\\Mobaxterm";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Mobaxterm";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Mobaxterm") },
        RegValue { name: "Version",     value: String::from("v4.1.10") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Mobaxterm") },
        RegValue { name: "Publisher",       value: String::from("Mobaxterm") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Mobaxterm\\uninstall.exe\"") },
    ]
}
