use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use crate::error::Result;
use crate::utils::http::HttpClient;
use crate::utils::signature::SignatureBuilder;

/// 腾讯云API客户端
pub struct TencentCloudClient {
    secret_id: String,
    secret_key: String,
    http_client: HttpClient,
}

impl TencentCloudClient {
    /// 创建新的腾讯云API客户端
    pub fn new(secret_id: String, secret_key: String) -> Self {
        Self {
            secret_id,
            secret_key,
            http_client: HttpClient::new(),
        }
    }
    
    /// 发送请求到腾讯云API
    pub async fn request<T, R>(&self, action: &str, params: &T, service: &str, version: &str, region: Option<&str>) -> Result<R>
    where
        T: Serialize,
        R: DeserializeOwned,
    {
        // 获取服务域名
        let host = format!("{}.tencentcloudapi.com", service);
        
        // 将参数序列化为JSON
        let mut json_params = serde_json::to_value(params).unwrap_or(json!({}));
        
        // 确保json_params是一个对象
        if !json_params.is_object() {
            json_params = json!({});
        }
        
        // 序列化为字符串
        let payload = json_params.to_string();
        
        // 构建签名
        let mut builder = SignatureBuilder::new(
            self.secret_id.clone(),
            self.secret_key.clone(),
            service.to_string(),
            host,
        )
        .action(action.to_string())
        .version(version.to_string())
        .payload(payload);
        
        // 如果提供了区域，则设置区域
        if let Some(region_value) = region {
            builder = builder.region(region_value.to_string());
        }
        
        // 生成签名信息
        let signature_info = builder.build();
        
        // 发送请求并解析响应
        self.http_client.send_request(signature_info).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_client() {
        let client = TencentCloudClient::new("test_id".to_string(), "test_key".to_string());
        assert_eq!(client.secret_id, "test_id");
        assert_eq!(client.secret_key, "test_key");
    }
} 