add_rules("mode.debug", "mode.release")

add_requires("cargo::aes", {configs = {cargo_toml = path.join(os.scriptdir(), "Cargo.toml")}})

target("aes-rs")
    set_kind("binary")
    add_files("src/main.rs")
    add_packages("cargo::aes")
