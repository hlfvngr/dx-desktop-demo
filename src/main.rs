#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use dx_desktop_demo::codec::Codec;
use dx_desktop_demo::rust_crypt_codec::AesCodec;

#[cfg_attr(feature = "bundle", windows_subsystem = "windows")]
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        //.with_custom_head(r#"<link rel="stylesheet" href="tailwind.css">"#.to_string())
        .with_custom_head(r#"<script src="bootstrap.bundle.min.js"></script>"#.to_string())
        .with_custom_head(r#"<link href="bootstrap.min.css" rel="stylesheet">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let mut secret_key = use_signal(|| "r3i9o2l1vb0h1t6n".to_string());
    let mut plain = use_signal(|| "".to_string());
    let mut cipher = use_signal(|| "".to_string());

    rsx! {

        div {
            class: "container flex-direction:column",
            div { class: "row",
                label{color: "black",
                    "密钥: "
                }
                input {
                    class: "form-control",
                    id: "secret_key_input",
                    r#type: "text",
                    value: "{secret_key}",
                    oninput: move |e| secret_key.set(e.value()),

                }
            }
            div {class: "row",

                div {class : "col-6",
                    div{ class: "row",
                        div {
                            class: "col-6",
                            label{style: "color: black;","明文: "}
                        }
                        div {
                            class: "col-6",
                            div {
                                class: "btn-group",
                                button {
                                    class: "btn btn-success btn-sm",
                                    r#type: "button",
                                    onclick: move |_| {
                                        let b = plain.read();
                                        let mut aes_codec = AesCodec::new(secret_key.read().to_string());
                                        let a = aes_codec.encode(b.as_str());
                                        cipher.set(a.unwrap());
                                    },
                                    "加密"
                                }
                                button {
                                    class: "btn btn-secondary btn-sm",
                                    r#type: "button",
                                    onclick: move |_| {
                                        plain.take();
                                    },
                                    "清空"
                                }
                            }
                        }
                    }
                    div{
                        textarea {
                            class: "form-control",
                            rows: 10,
                            id: "plaintext",
                            placeholder: "请输入明文",
                            value: "{plain}",
                            onchange: move |e| {
                                plain.set(e.value());
                            }
                        }
                    }
                }
                div {class : "col-6",
                    div{ class: "row",
                        div {
                            class: "col-6",
                            label{style: "color: black;","密文: "}
                        }
                        div{
                            class: "col-6",
                            div {
                                class: "btn-group",
                                button {
                                    class: "btn btn-success btn-sm",
                                    r#type: "button",
                                    onclick: move |_| {
                                        let b = cipher.read();
                                        let mut aes_codec = AesCodec::new(secret_key.read().to_string());
                                        let a = aes_codec.decode(b.as_str());
                                        plain.set(a.unwrap());
                                    },
                                    "解密"
                                }
                                button {
                                    class: "btn btn-secondary btn-sm",
                                    r#type: "button",
                                    onclick: move |_| {
                                        cipher.take();
                                    },
                                    "清空"
                                }
                            }

                        }
                    }
                    div{
                        textarea {
                            class: "form-control",
                            rows: 10,
                            id: "ciphertext",
                            placeholder: "请输入密文",
                            value: "{cipher}",
                            onchange: move |e| {
                                cipher.set(e.value());
                            }
                        }
                    }

                }


            }
        }

    }
}
