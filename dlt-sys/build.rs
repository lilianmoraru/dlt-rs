extern crate dotenv;
extern crate cmake;

use std::string::String;
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

type CMakeOptions = HashMap<String, String>;

fn main() {
    // Making sure that the `dlt-daemon` submodule is available when trying to compile it
    if !Path::new("dlt-daemon/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status()
                                   .expect("Failed to initialize the \"dlt-daemon\" submodule. \
                                            Make sure you have \"Git\" installed or that you don't \
                                            have issues with the internet(cloning from GitHub)");
    }

    // Register `dlt_sys` default values for the DLT CMake flags
    let mut cmake_options = register_cmake_defaults();

    // Loading from `.env` user-preferred `dlt_sys` configurations
    dotenv::dotenv().ok();

    // Overwrite default configurations with user-selected configurations
    for (variable_name, value) in std::env::vars() {
        // All of our variables start with `DLT_SYS_`, skip the others
        if variable_name.starts_with("DLT_SYS_") {
            if variable_name.starts_with("DLT_SYS_WITH_")
                || variable_name == "DLT_SYS_BUILD_SHARED_LIBS"
            {
                validate_cmake_option(&variable_name, &value);
            }

            // If the key exists, than it's an option we are interested passing to CMake
            let key_exists = cmake_options.contains_key(&variable_name);
            if key_exists {
                // Overwriting the default value
                cmake_options.insert(variable_name, value);
            }
        }
    }

    let cmake_options = cmake_options;
    configure_dlt_features(&cmake_options);

    let mut dst = cmake::Config::new("dlt-daemon");
    for (key, value) in &cmake_options {
        // Removing the "DLT_SYS_" prefix before passing the configurations to CMake
        let (_, key) = key.split_at(8);
        dst.define(key, &value);
    }

    let dst = dst.build();

    if is_cmake_option_activated(&cmake_options, "DLT_SYS_BUILD_SHARED_LIBS") {
        println!("cargo:rustc-link-search=native={}/build/dlt-build/lib", dst.display());
        println!("cargo:rustc-link-lib=dlt");
    } else {
        println!("cargo:rustc-link-search=native={}/build/dlt-build/lib/static", dst.display());
        println!("cargo:rustc-link-lib=static=dlt");
    }

    std::fs::copy(format!("{}/build/dlt-build/bin/dlt-daemon", dst.display()),
                  format!("{}/../../../dlt-daemon", dst.display())).unwrap();
}

//          =============== Helpers ===============
fn register_cmake_defaults() -> CMakeOptions {
    use std::str::FromStr;

    let mut cmake_options = HashMap::new();
    for &(key, value) in
        &[
            ("DLT_SYS_WITH_SYSTEMD",             "OFF"),
            ("DLT_SYS_WITH_SYSTEMD_WATCHDOG",    "OFF"),
            ("DLT_SYS_WITH_SYSTEMD_JOURNAL",     "OFF"),
            ("DLT_SYS_WITH_DOC",                 "OFF"),
            ("DLT_SYS_WITH_MAN",                 "OFF"),
            ("DLT_SYS_WTIH_DLT_ADAPTOR",         "OFF"), // Yes, "WTIH" :)
            ("DLT_SYS_WITH_DLT_ADAPTOR",         "OFF"), // Lets add this in case the flag is fixed
            ("DLT_SYS_WITH_DLT_CONSOLE",         "OFF"),
            ("DLT_SYS_WITH_DLT_EXAMPLES",        "OFF"),
            ("DLT_SYS_WITH_DLT_SYSTEM",          "OFF"),
            ("DLT_SYS_WITH_DLT_DBUS",            "OFF"),
            ("DLT_SYS_WITH_DLT_TESTS",           "OFF"),
            ("DLT_SYS_WITH_DLT_UNIT_TESTS",      "OFF"),
            ("DLT_SYS_WITH_DLT_SHM_ENABLE",      "OFF"),
            ("DLT_SYS_WITH_DLTTEST",             "OFF"),
            ("DLT_SYS_WITH_DLT_CXX11_EXT",       "OFF"),
            ("DLT_SYS_WITH_DLT_COREDUMPHANDLER", "OFF"),
            ("DLT_SYS_WITH_DLT_KPI",             "OFF"),
            ("DLT_SYS_WITH_CHECK_CONFIG_FILE",   "OFF"),
            ("DLT_SYS_WITH_TESTSCRIPTS",         "OFF"),
            ("DLT_SYS_WITH_GPROF",               "OFF"),
            ("DLT_SYS_WITH_DLT_USE_IPv6",        "ON"),
            ("DLT_SYS_DLT_USER",                 "Rustacean"),
            ("DLT_SYS_BUILD_SHARED_LIBS",        "OFF"),
            ("DLT_SYS_CMAKE_INSTALL_PREFIX",     "dlt-build"),
            ("DLT_SYS_CMAKE_BUILD_TYPE",         "Release")
        ]
    {
        if !key.starts_with("DLT_SYS_") {
            panic!("Incorrect dlt_sys CMake option(variable name): {}", key);
        }

        if key.starts_with("DLT_SYS_WITH_") || key == "DLT_SYS_BUILD_SHARED_LIBS" {
            validate_cmake_option(key, value);
        }

        let key   = String::from_str(key).ok();
        let value = String::from_str(value).ok();

        if let (Some(key), Some(value)) = (key, value) {
            cmake_options.insert(key, value);
        }
    }

    cmake_options
}

fn validate_cmake_option(key: &str, value: &str) {
    let expected_options = ["0", "1", "FALSE", "N", "NO", "OFF", "ON", "TRUE", "Y", "YES"];
    if let Err(_) = expected_options.binary_search(&value) {
        panic!("Invalid option for the key \"{}\".\n\
        Expected one of: {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n\
        Found: {}", key, "1", "0", "ON", "OFF", "YES", "NO", "Y", "N", "TRUE", "FALSE", value);
    }
}

fn is_cmake_value_on(value: &str) -> bool {
    let expected_options = ["1", "ON", "TRUE", "Y", "YES"];
    if let Ok(_) = expected_options.binary_search(&value) {
        true
    } else {
        false
    }
}

fn is_cmake_option_activated(cmake_options: &CMakeOptions, key: &str) -> bool {
    if let Some(value) = cmake_options.get(key) {
        return is_cmake_value_on(value.as_str());
    }

    false
}

fn configure_dlt_features(cmake_options: &CMakeOptions) {
    fn add_cfg(cfg: &str) {
        println!("cargo:rustc-cfg={}", cfg);
    }

    for (key, value) in cmake_options {
        if is_cmake_value_on(value.as_str()) {
            // Follows the order of the checks in the CMakeLists.txt file
            match key.as_str() {
                "DLT_SYS_WITH_DLT_SYSTEM" => {}, // find_package(ZLIB REQUIRED)
                "DLT_SYS_WITH_DLT_DBUS" => {}, // pkg_check_modules(DBUS REQUIRED dbus-1)
                "DLT_SYS_WITH_DLTTEST" => {}, // add_definitions
                "DLT_SYS_WITH_DLT_UNIT_TESTS" => {}, // add_definitions
                "DLT_SYS_WITH_DLT_SHM_ENABLE" => {}, // add_definitions
                "DLT_SYS_WITH_DLT_USE_IPv6" => {}, // add_definitions
                "DLT_SYS_WITH_GPROF" => {}, // CMAKE_C_FLAGS = "-pg"
                "DLT_SYS_WITH_DOC" => {},
                "DLT_SYS_WITH_DLT_CXX11_EXT" => {},
                "DLT_SYS_CMAKE_INSTALL_PREFIX" => {},
                "DLT_SYS_WITH_SYSTEMD" => {},
                "DLT_SYS_WITH_SYSTEMD_WATCHDOG" => {},
                "DLT_SYS_WITH_SYSTEMD_JOURNAL" => {},
                "DLT_SYS_WITH_DLT_UNIT_TESTS" => {},
                _ => {}
            }
        }
    }
}
