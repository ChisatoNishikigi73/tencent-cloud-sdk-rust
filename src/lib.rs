#![allow(non_snake_case)]

//! 腾讯云API SDK
//! 
//! 本库提供与腾讯云API进行交互的Rust实现

pub mod client;
pub mod error;
pub mod services;
pub mod utils;

// 重新导出
pub use client::TencentCloudClient;
pub use error::Error;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
