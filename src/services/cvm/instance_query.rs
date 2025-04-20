//! 实例查询相关接口
//! 
//! 包含查询实例列表、查询实例状态等查询接口

use serde::{Deserialize, Serialize};

use crate::client::TencentCloudClient;
use crate::error::Result;
use crate::services::cvm::instance::{ApiResponse, Filter, Instance};

/// 查询实例列表的请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstancesRequest {
    /// 按照一个或者多个实例ID查询。实例ID例如：ins-xxxxxxxx。
    /// 每次请求的实例的上限为100。参数不支持同时指定InstanceIds和Filters。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceIds: Option<Vec<String>>,
    
    /// 过滤条件。参数不支持同时指定InstanceIds和Filters。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Filters: Option<Vec<Filter>>,
    
    /// 偏移量，默认为0。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Offset: Option<i32>,
    
    /// 返回数量，默认为20，最大值为100。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Limit: Option<i32>,
}

/// 查询实例列表的响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstancesResponse {
    /// 符合条件的实例数量
    pub TotalCount: i32,
    
    /// 实例详情列表
    pub InstanceSet: Vec<Instance>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询实例列表响应类型
pub type DescribeInstancesResponseType = ApiResponse<DescribeInstancesResponse>;

/// 查询实例状态列表请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstancesStatusRequest {
    /// 按照一个或者多个实例ID查询。实例ID形如：ins-11112222。此参数的具体格式可参考API简介的ids.N一节。
    /// 每次请求的实例的上限为100。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceIds: Option<Vec<String>>,
    
    /// 偏移量，默认为0。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Offset: Option<i32>,
    
    /// 返回数量，默认为20，最大值为100。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Limit: Option<i32>,
}

/// 实例状态
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceStatus {
    /// 实例ID
    pub InstanceId: String,
    
    /// 实例状态。取值范围：
    /// PENDING：表示创建中
    /// LAUNCH_FAILED：表示创建失败
    /// RUNNING：表示运行中
    /// STOPPED：表示关机
    /// STARTING：表示开机中
    /// STOPPING：表示关机中
    /// REBOOTING：表示重启中
    /// SHUTDOWN：表示停止待销毁
    /// TERMINATING：表示销毁中
    pub InstanceState: String,
}

/// 查询实例状态列表响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstancesStatusResponse {
    /// 符合条件的实例状态数量
    pub TotalCount: i32,
    
    /// 实例状态列表
    pub InstanceStatusSet: Vec<InstanceStatus>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询实例状态列表响应类型
pub type DescribeInstancesStatusResponseType = ApiResponse<DescribeInstancesStatusResponse>;

/// 查询实例可调整配置请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstancesModificationRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。
    /// 每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 当前实例状态。默认为RUNNING。
    /// 取值范围：
    /// RUNNING：表示实例运行中
    /// STOPPED：表示实例已关机
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Status: Option<String>,
}

/// 实例可调整配置
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceTypeConfig {
    /// 实例机型
    pub InstanceType: String,
    
    /// CPU核数
    pub CPU: i32,
    
    /// 内存大小，单位：GB
    pub Memory: i32,
    
    /// 实例机型状态。取值范围：
    /// SELL：表示该实例机型处于售卖状态
    /// SOLD_OUT：表示该实例机型已售罄
    pub Status: String,
}

/// 实例可调整配置详情
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceTypeQuotaItem {
    /// 实例所在区域。
    pub Zone: String,
    
    /// 实例ID。
    pub InstanceId: String,
    
    /// 实例当前配置
    pub InstanceType: String,
    
    /// 可调整的实例规格列表
    pub InstanceTypeConfigSet: Vec<InstanceTypeConfig>,
}

/// 查询实例可调整配置响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstancesModificationResponse {
    /// 实例调整的配置详情
    pub InstanceModificationQuotaSet: Vec<InstanceTypeQuotaItem>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询实例可调整配置响应类型
pub type DescribeInstancesModificationResponseType = ApiResponse<DescribeInstancesModificationResponse>;

/// 查询实例操作限制请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstancesOperationLimitRequest {
    /// 按照一个或者多个实例ID查询。实例ID形如：ins-xxxxxxxx。
    /// 此参数的具体格式可参考API简介的ids.N一节。
    /// 每次请求的实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例操作。
    /// 取值范围：
    /// INSTANCE_DEGRADE：实例降配操作
    /// INTERNET_CHARGE_TYPE_CHANGE：修改网络带宽计费模式
    pub Operation: String,
}

/// 实例操作限制明细
#[derive(Debug, Clone, Deserialize)]
pub struct OperationDetail {
    /// 实例ID
    pub InstanceId: String,
    
    /// 实例操作限制详情
    pub DisasterRecoverGroupIds: Vec<String>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询实例操作限制响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstancesOperationLimitResponse {
    /// 该参数表示调整配置操作（降配）限制次数。
    pub InstanceOperationLimitSet: Vec<OperationDetail>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询实例操作限制响应类型
pub type DescribeInstancesOperationLimitResponseType = ApiResponse<DescribeInstancesOperationLimitResponse>;

/// 查询所支持的实例机型族信息请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstanceFamilyConfigsRequest {}

/// 机型族配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceFamilyConfig {
    /// 机型族名称的中文全称
    pub InstanceFamilyName: String,
    
    /// 机型族名称的英文简称
    pub InstanceFamily: String,
}

/// 查询所支持的实例机型族信息响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstanceFamilyConfigsResponse {
    /// 实例机型族配置列表
    pub InstanceFamilyConfigSet: Vec<InstanceFamilyConfig>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询所支持的实例机型族信息响应类型
pub type DescribeInstanceFamilyConfigsResponseType = ApiResponse<DescribeInstanceFamilyConfigsResponse>;

/// 查询实例机型列表请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstanceTypeConfigsRequest {
    /// 过滤条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Filters: Option<Vec<Filter>>,
}

/// 实例机型配置详情
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceTypeConfigItem {
    /// 可用区
    pub Zone: String,
    
    /// 实例机型
    pub InstanceType: String,
    
    /// 实例机型系列
    pub InstanceFamily: String,
    
    /// CPU核数
    pub CPU: i32,
    
    /// 内存大小，单位：GB
    pub Memory: i32,
    
    /// 是否支持GPU
    pub GPU: i32,
    
    /// 本地存储块数量
    pub FPGA: i32,
}

/// 查询实例机型列表响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstanceTypeConfigsResponse {
    /// 实例机型配置列表
    pub InstanceTypeConfigSet: Vec<InstanceTypeConfigItem>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询实例机型列表响应类型
pub type DescribeInstanceTypeConfigsResponseType = ApiResponse<DescribeInstanceTypeConfigsResponse>;

/// 实例查询服务
pub struct InstanceQueryService<'a> {
    client: &'a TencentCloudClient,
}

impl<'a> InstanceQueryService<'a> {
    /// 创建新的实例查询服务
    pub fn new(client: &'a TencentCloudClient) -> Self {
        Self { client }
    }

    /// 查询实例列表
    /// 
    /// 本接口 (DescribeInstances) 用于查询一个或多个实例的详细信息。
    /// 
    /// - 可以根据实例ID、实例名称或者实例计费模式等信息来查询实例的详细信息
    /// - 如果参数为空，返回当前用户一定数量（Limit所指定的数量，默认为20）的实例
    /// - 支持查询实例的最新操作（LatestOperation）以及最新操作状态(LatestOperationState)
    pub async fn describe_instances(&self, request: &DescribeInstancesRequest, region: &str) -> Result<DescribeInstancesResponseType> {
        self.client.request(
            "DescribeInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 查看实例状态列表
    /// 
    /// 本接口 (DescribeInstancesStatus) 用于查询一个或多个实例的状态。
    /// 
    /// - 可以根据实例ID来查询实例的状态
    /// - 如果参数为空，返回当前用户一定数量（Limit所指定的数量，默认为20）的实例状态
    pub async fn describe_instances_status(&self, request: &DescribeInstancesStatusRequest, region: &str) -> Result<DescribeInstancesStatusResponseType> {
        self.client.request(
            "DescribeInstancesStatus", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 查询实例可调整配置
    /// 
    /// 本接口 (DescribeInstancesModification) 用于查询实例可调整的机型配置。
    pub async fn describe_instances_modification(&self, request: &DescribeInstancesModificationRequest, region: &str) -> Result<DescribeInstancesModificationResponseType> {
        self.client.request(
            "DescribeInstancesModification", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 查询实例操作限制
    /// 
    /// 本接口用于查询实例操作限制信息。
    pub async fn describe_instances_operation_limit(&self, request: &DescribeInstancesOperationLimitRequest, region: &str) -> Result<DescribeInstancesOperationLimitResponseType> {
        self.client.request(
            "DescribeInstancesOperationLimit", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 查询所支持的实例机型族信息
    /// 
    /// 本接口 (DescribeInstanceFamilyConfigs) 查询当前用户和地域所支持的机型族信息。
    pub async fn describe_instance_family_configs(&self, request: &DescribeInstanceFamilyConfigsRequest, region: &str) -> Result<DescribeInstanceFamilyConfigsResponseType> {
        self.client.request(
            "DescribeInstanceFamilyConfigs", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 查询实例机型列表
    /// 
    /// 本接口 (DescribeInstanceTypeConfigs) 用于查询实例机型配置。
    /// 
    /// - 可以根据实例机型、实例族名称等信息来查询实例机型配置
    /// - 如果参数为空，返回当前用户地域所支持的所有机型配置
    pub async fn describe_instance_type_configs(&self, request: &DescribeInstanceTypeConfigsRequest, region: &str) -> Result<DescribeInstanceTypeConfigsResponseType> {
        self.client.request(
            "DescribeInstanceTypeConfigs", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
} 