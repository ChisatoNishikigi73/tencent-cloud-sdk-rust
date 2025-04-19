//! 地域相关接口
//! 
//! 包含查询地域列表和可用区列表的接口

use serde::{Deserialize, Serialize};

use crate::client::TencentCloudClient;
use crate::error::Result;

/// 地域信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Region {
    /// 地域名称，例如，ap-guangzhou
    pub Region: String,
    /// 地域描述，例如，华南地区(广州)
    pub RegionName: String,
    /// 地域是否可用状态
    pub RegionState: String,
}

/// 可用区信息
#[derive(Debug, Deserialize, Serialize)]
pub struct Zone {
    /// 可用区名称，例如，ap-guangzhou-1
    pub Zone: String,
    /// 可用区描述，例如，广州一区
    pub ZoneName: String,
    /// 可用区ID
    pub ZoneId: String,
    /// 可用区状态，包含AVAILABLE和UNAVAILABLE
    pub ZoneState: String,
}

/// API响应通用结构
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// 响应数据
    pub Response: T,
}

/// API错误响应
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    /// 错误信息
    pub Error: ApiError,
    /// 请求ID
    pub RequestId: String,
}

/// API错误
#[derive(Debug, Deserialize)]
pub struct ApiError {
    /// 错误代码
    pub Code: String,
    /// 错误消息
    pub Message: String,
}

/// 查询地域列表响应数据
#[derive(Debug, Deserialize)]
pub struct DescribeRegionsResponseData {
    /// 地域列表
    pub RegionSet: Vec<Region>,
    /// 唯一请求ID
    pub RequestId: String,
}

/// 查询可用区列表响应数据
#[derive(Debug, Deserialize)]
pub struct DescribeZonesResponseData {
    /// 可用区列表
    pub ZoneSet: Vec<Zone>,
    /// 唯一请求ID
    pub RequestId: String,
}

/// 完整的地域响应类型
pub type DescribeRegionsResponse = ApiResponse<DescribeRegionsResponseData>;

/// 完整的可用区响应类型
pub type DescribeZonesResponse = ApiResponse<DescribeZonesResponseData>;

/// 错误响应类型
pub type ErrorResponseType = ApiResponse<ErrorResponse>;

/// 查询可用区的请求参数
#[derive(Debug, Serialize)]
pub struct DescribeZonesRequest {
    /// 地域ID
    pub Region: String,
}

/// 地域服务
pub struct RegionService<'a> {
    client: &'a TencentCloudClient,
}

impl<'a> RegionService<'a> {
    /// 创建新的地域服务
    pub fn new(client: &'a TencentCloudClient) -> Self {
        Self { client }
    }

    /// 查询地域列表
    /// 
    /// 本接口(DescribeRegions)用于查询地域列表。
    pub async fn describe_regions(&self) -> Result<DescribeRegionsResponse> {
        // 空参数请求
        let params = ();
        
        // 使用client发送请求
        self.client.request(
            "DescribeRegions", 
            &params, 
            "cvm", 
            "2017-03-12", 
            None
        ).await
    }

    /// 查询可用区列表
    /// 
    /// 本接口(DescribeZones)用于查询可用区列表。
    /// 需要指定地域参数。
    pub async fn describe_zones(&self, region: &str) -> Result<DescribeZonesResponse> {
        // 空参数请求 - Region参数通过最后一个参数传递
        let params = ();
        
        // 使用client发送请求，将region作为第5个参数传递
        self.client.request(
            "DescribeZones", 
            &params, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
} 