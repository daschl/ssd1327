//! SSD1327 Display Driver
#![cfg_attr(not(test), no_std)]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/ssd1327/0.1.0")]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod command;
pub mod display;
