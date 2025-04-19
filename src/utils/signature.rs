use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// 获取当前UTC日期，格式为YYYY-MM-DD
pub fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    now.format("%Y-%m-%d").to_string()
}

/// 获取当前时间戳（秒）
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("获取时间失败")
        .as_secs()
}

/// 计算SHA256哈希值并返回十六进制字符串
pub fn sha256_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

/// 使用HMAC-SHA256算法计算消息认证码
fn hmac_sha256(key: &[u8], data: &str) -> Vec<u8> {
    let mut mac = HmacSha256::new_from_slice(key)
        .expect("HMAC初始化失败");
    mac.update(data.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

/// 计算HMAC-SHA256并转换为十六进制字符串
pub fn hmac_sha256_hex(key: &[u8], data: &str) -> String {
    let hmac_result = hmac_sha256(key, data);
    hex::encode(hmac_result)
}

/// 生成腾讯云API的签名信息
pub struct SignatureBuilder {
    secret_id: String,
    secret_key: String,
    service: String,
    host: String,
    region: String,
    action: String,
    version: String,
    timestamp: u64,
    payload: String,
}

impl SignatureBuilder {
    /// 创建新的签名生成器
    pub fn new(
        secret_id: String, 
        secret_key: String, 
        service: String,
        host: String,
    ) -> Self {
        Self {
            secret_id,
            secret_key,
            service,
            host,
            region: String::new(),
            action: String::new(),
            version: String::new(),
            timestamp: get_timestamp(),
            payload: String::from("{}"),
        }
    }

    /// 设置区域
    pub fn region(mut self, region: String) -> Self {
        self.region = region;
        self
    }

    /// 设置API操作
    pub fn action(mut self, action: String) -> Self {
        self.action = action;
        self
    }

    /// 设置API版本
    pub fn version(mut self, version: String) -> Self {
        self.version = version;
        self
    }

    /// 设置请求载荷（JSON字符串）
    pub fn payload(mut self, payload: String) -> Self {
        self.payload = payload;
        self
    }

    /// 设置时间戳（主要用于测试）
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    /// 构建签名和相关HTTP请求头
    pub fn build(&self) -> SignatureInfo {
        // 获取日期
        let date = get_date();
        let timestamp_str = self.timestamp.to_string();

        // 步骤1：拼接规范请求串
        let http_request_method = "POST";
        let canonical_uri = "/";
        let canonical_query_string = "";
        let canonical_headers = format!("content-type:application/json; charset=utf-8\nhost:{}\n", self.host);
        let signed_headers = "content-type;host";
        let hashed_request_payload = sha256_hex(&self.payload);
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            http_request_method,
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            hashed_request_payload
        );

        // 步骤2：拼接待签名字符串
        let algorithm = "TC3-HMAC-SHA256";
        let credential_scope = format!("{}/{}/tc3_request", date, self.service);
        let hashed_canonical_request = sha256_hex(&canonical_request);
        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm,
            timestamp_str,
            credential_scope,
            hashed_canonical_request
        );

        // 步骤3：计算签名
        let k_key = format!("TC3{}", self.secret_key);
        let k_date = hmac_sha256(k_key.as_bytes(), &date);
        let k_service = hmac_sha256(&k_date, &self.service);
        let k_signing = hmac_sha256(&k_service, "tc3_request");
        let signature = hmac_sha256_hex(&k_signing, &string_to_sign);

        // 步骤4：拼接Authorization
        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm,
            self.secret_id,
            credential_scope,
            signed_headers,
            signature
        );

        // 构建返回信息
        SignatureInfo {
            url: format!("https://{}", self.host),
            headers: vec![
                ("Authorization".to_string(), authorization),
                ("Content-Type".to_string(), "application/json; charset=utf-8".to_string()),
                ("Host".to_string(), self.host.clone()),
                ("X-TC-Action".to_string(), self.action.clone()),
                ("X-TC-Timestamp".to_string(), timestamp_str),
                ("X-TC-Version".to_string(), self.version.clone()),
                ("X-TC-Region".to_string(), self.region.clone()),
            ],
            payload: self.payload.clone(),
        }
    }
}

/// 签名信息和HTTP请求需要的数据
pub struct SignatureInfo {
    /// 请求URL
    pub url: String,
    /// HTTP请求头
    pub headers: Vec<(String, String)>,
    /// 请求体
    pub payload: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_hex() {
        let result = sha256_hex("test");
        assert_eq!(result, "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    }

    #[test]
    fn test_hmac_sha256_hex() {
        let key = b"key";
        let data = "The quick brown fox jumps over the lazy dog";
        let result = hmac_sha256_hex(key, data);
        assert_eq!(result, "f7bc83f430538424b13298e6aa6fb143ef4d59a14946175997479dbc2d1a3cd8");
    }
} 