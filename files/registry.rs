//! Windows registry read helpers for Fabfilter Micro.

pub const APP_KEY: &str = "Software\\Fabfilter\\Fabfilter Micro";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Fabfilter Micro";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Fabfilter Micro") },
        RegValue { name: "Version",     value: String::from("v1.2.13") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Fabfilter Micro") },
        RegValue { name: "Publisher",       value: String::from("Fabfilter") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Fabfilter Micro\\uninstall.exe\"") },
    ]
}
