load("@prelude//rust:cargo_buildscript.bzl", "buildscript_run")
load("@prelude//rust:cargo_package.bzl", "cargo")

cargo.rust_library(
    name = "rustyvault",
    srcs = glob(["src/**/*.rs"]),
    crate_root = "src/lib.rs",
    edition = "2024",
    env = {
        "CARGO_BIN_NAME": "rvault",
        "CARGO_PKG_AUTHORS": "",
        "CARGO_PKG_DESCRIPTION": "RustyVault is a powerful identity-based secrets management software, providing features such as\ncryptographic key management, encryption as a service, public key cryptography, certificates management, identity credentials\nmanagement and so forth.\n\nRustyVault's RESTful API is designed to be fully compatible with Hashicorp Vault.\n",
        "CARGO_PKG_NAME": "rusty_vault",
        "CARGO_PKG_REPOSITORY": "https://github.com/Tongsuo-Project/RustyVault",
        "CARGO_PKG_VERSION": "0.2.1",
        "CARGO_PKG_VERSION_MAJOR": "0",
        "CARGO_PKG_VERSION_MINOR": "2",
        "CARGO_PKG_VERSION_PATCH": "1",
    },
    features = [
        "crypto_adaptor_openssl",
        "default",
    ],
    platform = {},
    deps = [
        "//third-party:actix-rt-2.10.0",
        "//third-party:actix-tls-3.4.0",
        "//third-party:actix-web-4.9.0",
        "//third-party:anyhow-1.0.93",
        "//third-party:as-any-0.3.1",
        "//third-party:async-trait-0.1.86",
        "//third-party:base64-0.22.1",
        "//third-party:bcrypt-0.15.1",
        "//third-party:better_default-1.0.5",
        "//third-party:blake2b_simd-1.0.3",
        "//third-party:build-time-0.1.3",
        "//third-party:chrono-0.4.39",
        "//third-party:clap-4.5.28",
        "//third-party:ctor-0.2.9",
        "//third-party:daemonize-0.5.0",
        "//third-party:dashmap-5.5.3",
        "//third-party:delay_timer-0.11.6",
        "//third-party:derivative-2.2.0",
        "//third-party:derive_more-0.99.18",
        "//third-party:enum-map-2.7.3",
        "//third-party:env_logger-0.10.2",
        "//third-party:foreign-types-0.3.2",
        "//third-party:glob-0.3.1",
        "//third-party:go-defer-0.1.0",
        "//third-party:hcl-rs-0.16.9",
        "//third-party:hex-0.4.3",
        "//third-party:humantime-2.1.0",
        "//third-party:ipnetwork-0.20.0",
        "//third-party:lazy_static-1.5.0",
        "//third-party:libc-0.2.169",
        "//third-party:log-0.4.22",
        "//third-party:openssl-0.10.71",
        "//third-party:openssl-sys-0.9.106",
        "//third-party:pem-3.0.4",
        "//third-party:prettytable-0.10.0",
        "//third-party:prometheus-client-0.22.3",
        "//third-party:radix_trie-0.2.1",
        "//third-party:rand-0.8.5",
        "//third-party:regex-1.11.1",
        "//third-party:rpassword-7.3.1",
        "//third-party:rustls-0.23.22",
        "//third-party:rustls-pemfile-2.1.3",
        "//third-party:serde-1.0.217",
        "//third-party:serde_bytes-0.11.15",
        "//third-party:serde_derive-1.0.217",
        "//third-party:serde_json-1.0.138",
        "//third-party:serde_yaml-0.9.34+deprecated",
        "//third-party:strum-0.25.0",
        "//third-party:strum_macros-0.25.3",
        "//third-party:sysexits-0.7.14",
        "//third-party:sysinfo-0.31.4",
        "//third-party:thiserror-1.0.65",
        "//third-party:tokio-1.43.0",
        "//third-party:ureq-2.10.1",
        "//third-party:url-2.5.3",
        "//third-party:webpki-roots-0.26.3",
        "//third-party:zeroize-1.8.1",
    ],
    visibility = ["PUBLIC"],
)