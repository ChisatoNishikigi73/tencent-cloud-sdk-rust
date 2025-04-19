use reqwest::{self, Client, header};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

use crate::error::{Error, Result};
use super::signature::SignatureInfo;

/// HTTP客户端，处理腾讯云API请求
pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    /// 创建新的HTTP客户端
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("无法创建HTTP客户端");
        Self { client }
    }
    
    /// 发送请求并解析响应
    pub async fn send_request<T>(&self, signature_info: SignatureInfo) -> Result<T>
    where
        T: DeserializeOwned,
    {
        // 创建请求头
        let mut headers = header::HeaderMap::new();
        for (key, value) in signature_info.headers.iter() {
            let header_name = match header::HeaderName::from_bytes(key.as_bytes()) {
                Ok(name) => name,
                Err(e) => return Err(Error::HeaderError(format!("无效的头名称: {}", e))),
            };
            
            let header_value = match header::HeaderValue::from_str(value) {
                Ok(val) => val,
                Err(e) => return Err(Error::HeaderError(format!("无效的头值: {}", e))),
            };
            
            headers.insert(header_name, header_value);
        }
        
        // 打印请求信息
        // println!("请求URL: {}", signature_info.url);
        // println!("请求头: {:?}", headers);
        // println!("请求体: {}", signature_info.payload);
        
        // 发送请求
        let response = self.client
            .post(&signature_info.url)
            .headers(headers)
            .body(signature_info.payload.clone())
            .send()
            .await?;
        
        // 检查响应状态
        let status = response.status();
        println!("响应状态码: {}", status);
        
        // 获取并打印响应文本
        let response_text = response.text().await?;
        // println!("响应内容: {}", response_text);
        
        // 首先尝试将响应文本解析为JSON以检查是否有错误
        let json_value: Value = match serde_json::from_str(&response_text) {
            Ok(v) => v,
            Err(e) => {
                println!("无法解析JSON: {}", e);
                return Err(Error::SerdeError(e));
            }
        };
        
        // 检查响应中是否有错误信息
        if let Some(response) = json_value.get("Response") {
            if let Some(error) = response.get("Error") {
                if let (Some(code), Some(message)) = (error.get("Code"), error.get("Message")) {
                    if let (Some(code_str), Some(message_str)) = (code.as_str(), message.as_str()) {
                        println!("API错误: {} - {}", code_str, message_str);
                        return Err(Error::ApiError {
                            code: code_str.to_string(),
                            message: message_str.to_string(),
                        });
                    }
                }
            }
        }
        
        // 从文本重新解析JSON为请求的类型
        match serde_json::from_str::<T>(&response_text) {
            Ok(data) => Ok(data),
            Err(err) => {
                println!("JSON解析错误: {}", err);
                Err(Error::SerdeError(err))
            }
        }
    }
    
    /// 默认实例
    pub fn default() -> Self {
        Self::new()
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
} 