extern crate gcc;
extern crate cmake;

fn main() {
    let dst = cmake::Config::new("dlt-daemon")
                            .cxxflag("-fPIC")
                            .define("BUILD_SHARED_LIBS",    "OFF")
                            .define("WITH_MAN",             "OFF")
                            .define("WTIH_DLT_ADAPTOR",     "OFF")
                            .define("WITH_DLT_CONSOLE",     "OFF")
                            .define("WITH_DLT_SYSTEM",      "OFF")
                            .define("WITH_DLT_DBUS",        "OFF")
                            .define("WITH_DLT_EXAMPLES",    "OFF")
                            .define("WITH_DLT_TESTS",       "OFF")
                            .define("WITH_DLT_KPI",         "OFF")
                            .define("DLT_USER",             "Rustaceans")
                            .define("CMAKE_BUILD_TYPE",     "Release")
                            .define("CMAKE_INSTALL_PREFIX", "dlt-build")
                            .build();

    println!("cargo:rustc-link-search=native={}/build/dlt-build/lib/static", dst.display());
    println!("cargo:rustc-link-lib=static=dlt");

    std::fs::copy(format!("{}/build/dlt-build/bin/dlt-daemon", dst.display()),
                  format!("{}/../../../dlt-daemon", dst.display())).unwrap();
}
