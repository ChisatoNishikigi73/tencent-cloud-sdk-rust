//! 实例价格相关接口
//! 
//! 包含创建实例询价、续费实例询价、重装实例询价等询价接口

use serde::{Deserialize, Serialize};

use crate::client::TencentCloudClient;
use crate::error::Result;
use crate::services::cvm::instance::{ApiResponse, DataDisk, EnhancedService, InstanceChargePrepaid, InstanceChargeType, InternetAccessible, LoginSettings, Placement, SystemDisk, VirtualPrivateCloud};

/// 创建实例询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceRunInstancesRequest {
    /// 实例所在的位置。通过该参数可以指定实例所属可用区，所属项目，所属宿主机等属性。
    pub Placement: Placement,
    
    /// 指定有效的镜像ID，格式形如img-xxx。通过DescribeImages接口获取镜像信息。
    pub ImageId: String,
    
    /// 实例机型。不同实例机型指定了不同的资源规格，具体取值可通过调用接口DescribeInstanceTypeConfigs来获得最新的规格表或参见实例类型描述。
    pub InstanceType: String,
    
    /// 实例系统盘配置信息。若不指定该参数，则按照系统默认值进行分配。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SystemDisk: Option<SystemDisk>,
    
    /// 实例数据盘配置信息。若不指定该参数，则默认不购买数据盘。支持购买的数据盘类型详见类型说明。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DataDisks: Option<Vec<DataDisk>>,
    
    /// 公网带宽相关信息设置。若不指定该参数，则默认公网带宽为0Mbps。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InternetAccessible: Option<InternetAccessible>,
    
    /// 实例计费类型。
    /// PREPAID：预付费，即包年包月
    /// POSTPAID_BY_HOUR：按小时后付费
    /// CDHPAID：独享子机（基于专用宿主机创建，宿主机部分的资源不收费）
    /// SPOTPAID：竞价付费
    /// 默认值：POSTPAID_BY_HOUR。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargeType: Option<InstanceChargeType>,
    
    /// 预付费模式，即包年包月相关参数设置。通过该参数可以指定包年包月实例的购买时长、是否设置自动续费等属性。
    /// 若指定实例的付费模式为预付费则该参数必传。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargePrepaid: Option<InstanceChargePrepaid>,
    
    /// 购买实例数量。取值范围：[1，100]。默认取值：1。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceCount: Option<i32>,
    
    /// 实例的登录设置。通过该参数可以设置实例的登录方式密码、密钥或保持镜像的原始登录设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub LoginSettings: Option<LoginSettings>,
    
    /// 增强服务。通过该参数可以指定是否开启云安全、云监控等服务。若不指定该参数，则默认开启云监控、云安全服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub EnhancedService: Option<EnhancedService>,
    
    /// 私有网络相关信息配置。通过该参数可以指定私有网络的ID，子网ID等信息。若不指定该参数，则默认使用基础网络。若在此参数中指定了私有网络IP，表示每个实例的主网卡IP，并优先使用。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub VirtualPrivateCloud: Option<VirtualPrivateCloud>,
}

/// 价格详情
#[derive(Debug, Clone, Deserialize)]
pub struct Price {
    /// 描述了实例价格。
    pub InstancePrice: InstancePrice,
    
    /// 描述了网络价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub BandwidthPrice: Option<BandwidthPrice>,
}

/// 实例价格
#[derive(Debug, Clone, Deserialize)]
pub struct InstancePrice {
    /// 描述了按带宽计费的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPrice: Option<f64>,
    
    /// 描述了按流量计费的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ChargeUnit: Option<String>,
    
    /// 描述了购买实例的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OriginalPrice: Option<f64>,
    
    /// 折扣后的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountPrice: Option<f64>,
}

/// 带宽价格
#[derive(Debug, Clone, Deserialize)]
pub struct BandwidthPrice {
    /// 描述了按带宽计费的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPrice: Option<f64>,
    
    /// 描述了按流量计费的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ChargeUnit: Option<String>,
    
    /// 描述了带宽价格的折扣。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OriginalPrice: Option<f64>,
    
    /// 折扣后的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountPrice: Option<f64>,
}

/// 创建实例询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceRunInstancesResponse {
    /// 该参数表示对应配置实例的价格。
    pub Price: Price,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 创建实例询价响应类型
pub type InquiryPriceRunInstancesResponseType = ApiResponse<InquiryPriceRunInstancesResponse>;

/// 续费实例询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceRenewInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 预付费模式，即包年包月相关参数设置。通过该参数可以指定包年包月实例的续费时长、是否设置自动续费等属性。
    pub InstanceChargePrepaid: InstanceChargePrepaid,
    
    /// 是否续费弹性数据盘。取值范围：
    /// TRUE：表示续费包年包月实例同时续费其挂载的弹性数据盘
    /// FALSE：表示续费包年包月实例同时不再续费其挂载的弹性数据盘
    /// 默认取值：TRUE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RenewPortableDataDisk: Option<bool>,
}

/// 续费实例询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceRenewInstancesResponse {
    /// 该参数表示对应配置实例的价格。
    pub Price: Price,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 续费实例询价响应类型
pub type InquiryPriceRenewInstancesResponseType = ApiResponse<InquiryPriceRenewInstancesResponse>;

/// 重装实例询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceResetInstanceRequest {
    /// 实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。
    pub InstanceId: String,
    
    /// 指定有效的镜像ID，格式形如img-xxx。镜像类型分为四种：
    /// 公共镜像
    /// 自定义镜像
    /// 共享镜像
    /// 服务市场镜像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ImageId: Option<String>,
    
    /// 实例系统盘配置信息。系统盘为云盘的实例可以通过该参数指定重装后的系统盘大小。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SystemDisk: Option<SystemDisk>,
    
    /// 实例登录设置。通过该参数可以设置实例的登录方式密码、密钥或保持镜像的原始登录设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub LoginSettings: Option<LoginSettings>,
    
    /// 增强服务。通过该参数可以指定是否开启云安全、云监控等服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub EnhancedService: Option<EnhancedService>,
}

/// 重装实例询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceResetInstanceResponse {
    /// 该参数表示重装实例的价格。
    pub Price: Price,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 重装实例询价响应类型
pub type InquiryPriceResetInstanceResponseType = ApiResponse<InquiryPriceResetInstanceResponse>;

/// 调整实例配置询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceResizeInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例机型。不同实例机型指定了不同的资源规格，具体取值可通过调用接口DescribeInstanceTypeConfigs来获得最新的规格表或参见实例类型描述。
    pub InstanceType: String,
}

/// 调整实例配置询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceResizeInstancesResponse {
    /// 该参数表示调整实例配置的价格。
    pub Price: Price,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 调整实例配置询价响应类型
pub type InquiryPriceResizeInstancesResponseType = ApiResponse<InquiryPriceResizeInstancesResponse>;

/// 修改实例计费模式询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceModifyInstancesChargeTypeRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例计费类型。
    /// PREPAID：预付费，即包年包月
    /// POSTPAID_BY_HOUR：按小时后付费
    pub InstanceChargeType: InstanceChargeType,
    
    /// 预付费模式，即包年包月相关参数设置。通过该参数可以指定包年包月实例的购买时长、是否设置自动续费等属性。
    /// 若指定实例的付费模式为预付费则该参数必传。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargePrepaid: Option<InstanceChargePrepaid>,
}

/// 修改实例计费模式询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceModifyInstancesChargeTypeResponse {
    /// 该参数表示对应配置实例转换计费模式的价格。
    pub Price: Price,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 修改实例计费模式询价响应类型
pub type InquiryPriceModifyInstancesChargeTypeResponseType = ApiResponse<InquiryPriceModifyInstancesChargeTypeResponse>;

/// 扩容实例磁盘询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceResizeInstanceDisksRequest {
    /// 待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。
    pub InstanceId: String,
    
    /// 待扩容的数据盘配置信息。只支持扩容非弹性数据盘（连接在实例上的数据盘）。
    /// 数据盘容量单位：GB。
    /// 最小扩容步长：10G。
    pub DataDisks: Vec<DataDisk>,
    
    /// 是否对运行中的实例选择强制关机。
    /// TRUE：表示对运行中的实例强制关机，然后预挂载，再起机
    /// FALSE：表示如果实例在运行中，则无法扩容，且返回错误
    /// 默认值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ForceStop: Option<bool>,
}

/// 扩容实例磁盘询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceResizeInstanceDisksResponse {
    /// 该参数表示磁盘扩容成对应配置的价格。
    pub Price: Price,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 扩容实例磁盘询价响应类型
pub type InquiryPriceResizeInstanceDisksResponseType = ApiResponse<InquiryPriceResizeInstanceDisksResponse>;

/// 退还实例询价请求参数
#[derive(Debug, Clone, Serialize)]
pub struct InquiryPriceTerminateInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
}

/// 退还实例询价响应
#[derive(Debug, Deserialize)]
pub struct InquiryPriceTerminateInstancesResponse {
    /// 退款详情。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RefundSet: Option<Vec<ResourcePrice>>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 资源价格
#[derive(Debug, Clone, Deserialize)]
pub struct ResourcePrice {
    /// 资源名称
    pub ResourceId: String,
    
    /// 资源原价
    pub OriginalPrice: f64,
    
    /// 折扣后的价格
    pub DiscountPrice: f64,
}

/// 退还实例询价响应类型
pub type InquiryPriceTerminateInstancesResponseType = ApiResponse<InquiryPriceTerminateInstancesResponse>;

/// 实例价格相关服务
pub struct InstancePriceService<'a> {
    client: &'a TencentCloudClient,
}

impl<'a> InstancePriceService<'a> {
    /// 创建新的实例价格服务
    pub fn new(client: &'a TencentCloudClient) -> Self {
        Self { client }
    }

    /// 创建实例询价
    /// 
    /// 本接口(InquiryPriceRunInstances)用于创建实例询价。
    pub async fn inquiry_price_run_instances(&self, request: &InquiryPriceRunInstancesRequest, region: &str) -> Result<InquiryPriceRunInstancesResponseType> {
        self.client.request(
            "InquiryPriceRunInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 续费实例询价
    /// 
    /// 本接口(InquiryPriceRenewInstances)用于续费包年包月实例询价。
    /// 
    /// - 只支持查询包年包月实例的续费价格。
    pub async fn inquiry_price_renew_instances(&self, request: &InquiryPriceRenewInstancesRequest, region: &str) -> Result<InquiryPriceRenewInstancesResponseType> {
        self.client.request(
            "InquiryPriceRenewInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 重装实例询价
    /// 
    /// 本接口(InquiryPriceResetInstance)用于重装实例询价。
    /// 
    /// - 如果指定了ImageId参数，则使用指定的镜像进行重装询价；否则按照当前实例使用的镜像进行重装询价。
    /// - 目前只支持系统盘类型是CLOUD_BASIC、CLOUD_PREMIUM、CLOUD_SSD类型的实例使用该接口实现重装询价操作。
    /// - 目前不支持境外地域的实例使用该接口实现重装询价操作。
    pub async fn inquiry_price_reset_instance(&self, request: &InquiryPriceResetInstanceRequest, region: &str) -> Result<InquiryPriceResetInstanceResponseType> {
        self.client.request(
            "InquiryPriceResetInstance", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 调整实例配置询价
    /// 
    /// 本接口(InquiryPriceResizeInstancesType)用于调整实例配置询价。
    /// 
    /// - 目前只支持查询系统盘为云硬盘的实例配置变更的价格，且系统盘要挂载在实例上。
    /// - 目前不支持配置降级的价格查询。
    pub async fn inquiry_price_resize_instances(&self, request: &InquiryPriceResizeInstancesRequest, region: &str) -> Result<InquiryPriceResizeInstancesResponseType> {
        self.client.request(
            "InquiryPriceResizeInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 修改实例计费模式询价
    /// 
    /// 本接口(InquiryPriceModifyInstancesChargeType)用于修改实例计费模式询价。
    /// 
    /// - 目前只支持查询从包年包月转为按小时计费、以及从按小时计费转为包年包月的价格。
    pub async fn inquiry_price_modify_instances_charge_type(&self, request: &InquiryPriceModifyInstancesChargeTypeRequest, region: &str) -> Result<InquiryPriceModifyInstancesChargeTypeResponseType> {
        self.client.request(
            "InquiryPriceModifyInstancesChargeType", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 扩容实例磁盘询价
    /// 
    /// 本接口(InquiryPriceResizeInstanceDisks)用于扩容实例磁盘询价。
    /// 
    /// - 目前只支持扩容非弹性数据盘（挂载在实例上的数据盘）。
    /// - 目前不支持CDCPAID类型实例使用该接口扩容。
    pub async fn inquiry_price_resize_instance_disks(&self, request: &InquiryPriceResizeInstanceDisksRequest, region: &str) -> Result<InquiryPriceResizeInstanceDisksResponseType> {
        self.client.request(
            "InquiryPriceResizeInstanceDisks", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 退还实例询价
    /// 
    /// 本接口(InquiryPriceTerminateInstances)用于退还实例询价。
    /// 
    /// - 目前只支持查询包年包月实例的退还价格。
    /// - 只支持包年包月实例的退还询价。
    pub async fn inquiry_price_terminate_instances(&self, request: &InquiryPriceTerminateInstancesRequest, region: &str) -> Result<InquiryPriceTerminateInstancesResponseType> {
        self.client.request(
            "InquiryPriceTerminateInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
} 