//! 实例配置相关接口
//! 
//! 包含查询用户配额、调整实例配置等各种配置操作

use serde::{Deserialize, Serialize};

use crate::client::TencentCloudClient;
use crate::error::Result;
use crate::services::cvm::instance::{ApiResponse, InstanceChargePrepaid};

/// 查询用户配额详情请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeAccountQuotaRequest {}

/// 用户配额详情
#[derive(Debug, Clone, Deserialize)]
pub struct AccountQuota {
    /// 配额名称，取值范围：
    /// TOTAL_SPOT_INSTANCE_QUOTAS：竞价实例配额
    /// TOTAL_CVM_QUOTAS：CVM配额
    /// TOTAL_PREPAID_CVM_QUOTAS：预付费CVM配额
    /// TOTAL_POSTPAID_CVM_QUOTAS：后付费CVM配额
    pub QuotaId: String,
    
    /// 当前值
    pub QuotaCurrent: i32,
    
    /// 配额上限
    pub QuotaLimit: i32,
}

/// 查询用户配额详情响应
#[derive(Debug, Deserialize)]
pub struct DescribeAccountQuotaResponse {
    /// 用户配额详情
    pub AccountQuotaSet: Vec<AccountQuota>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 查询用户配额详情响应类型
pub type DescribeAccountQuotaResponseType = ApiResponse<DescribeAccountQuotaResponse>;

/// 创建定时任务请求参数
#[derive(Debug, Clone, Serialize)]
pub struct CreateDisasterRecoverGroupRequest {
    /// 分散置放群组名称，长度1-60个字符，支持中、英文。
    pub Name: String,
    
    /// 分散置放群组类型，取值范围：
    /// HOST：物理机
    /// RACK：机架
    /// SWITCH：交换机
    pub Type: String,
    
    /// 用于保证请求幂等性的字符串。该字符串由客户生成，需保证不同请求之间唯一，最大值不超过64个ASCII字符。若不指定该参数，则无法保证请求的幂等性。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ClientToken: Option<String>,
}

/// 创建定时任务响应
#[derive(Debug, Deserialize)]
pub struct CreateDisasterRecoverGroupResponse {
    /// 分散置放群组ID列表。
    pub DisasterRecoverGroupId: String,
    
    /// 分散置放群组类型，取值范围：
    /// HOST：物理机
    /// RACK：机架
    /// SWITCH：交换机
    pub Type: String,
    
    /// 分散置放群组名称，长度1-60个字符，支持中、英文。
    pub Name: String,
    
    /// 置放群组内可容纳的云服务器数量。
    pub CvmQuotaTotal: i32,
    
    /// 置放群组内已有的云服务器数量。
    pub CurrentNum: i32,
    
    /// 置放群组创建时间。
    pub CreateTime: String,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 创建定时任务响应类型
pub type CreateDisasterRecoverGroupResponseType = ApiResponse<CreateDisasterRecoverGroupResponse>;

/// 调整实例配置请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ResizeInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例机型。不同实例机型指定了不同的资源规格，具体取值可通过调用接口DescribeInstanceTypeConfigs来获得最新的规格表或参见实例类型描述。
    /// 目前支持自定义创建cos、hs、s5、sa2、sa3、hcc、hcr等类型的实例。
    pub InstanceType: String,
    
    /// 是否对运行中的实例选择强制关机。建议对运行中的实例先手动关机，然后再重置用户密码。取值范围：
    /// TRUE：表示在正常关机失败后进行强制关机
    /// FALSE：表示在正常关机失败后不进行强制关机
    /// 
    /// 默认取值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ForceStop: Option<bool>,
    
    /// 是否同时切换私有网络，仅基础网络切换为VPC网络时使用。取值范围：
    /// TRUE：表示同时切换私有网络
    /// FALSE：表示不同时切换私有网络
    /// 
    /// 默认取值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ReserveHostName: Option<bool>,
}

/// 调整实例配置响应
#[derive(Debug, Deserialize)]
pub struct ResizeInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 调整实例配置响应类型
pub type ResizeInstancesResponseType = ApiResponse<ResizeInstancesResponse>;

/// 修改实例所属项目请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ModifyInstancesProjectRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求允许操作的实例数量上限是100。
    pub InstanceIds: Vec<String>,
    
    /// 项目ID。项目可以使用AddProject接口创建。后续使用DescribeInstances接口查询实例时，项目ID可用于过滤结果。
    pub ProjectId: i32,
}

/// 修改实例所属项目响应
#[derive(Debug, Deserialize)]
pub struct ModifyInstancesProjectResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 修改实例所属项目响应类型
pub type ModifyInstancesProjectResponseType = ApiResponse<ModifyInstancesProjectResponse>;

/// 调整实例分散置放群组请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ModifyDisasterRecoverGroupAttributeRequest {
    /// 分散置放群组ID，可使用DescribeDisasterRecoverGroups接口获取。
    pub DisasterRecoverGroupId: String,
    
    /// 分散置放群组名称，长度1-60个字符，支持中、英文。
    pub Name: String,
}

/// 调整实例分散置放群组响应
#[derive(Debug, Deserialize)]
pub struct ModifyDisasterRecoverGroupAttributeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 调整实例分散置放群组响应类型
pub type ModifyDisasterRecoverGroupAttributeResponseType = ApiResponse<ModifyDisasterRecoverGroupAttributeResponse>;

/// 删除定时任务请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteDisasterRecoverGroupsRequest {
    /// 分散置放群组ID列表，可通过DescribeDisasterRecoverGroups接口获取。
    pub DisasterRecoverGroupIds: Vec<String>,
}

/// 删除定时任务响应
#[derive(Debug, Deserialize)]
pub struct DeleteDisasterRecoverGroupsResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 删除定时任务响应类型
pub type DeleteDisasterRecoverGroupsResponseType = ApiResponse<DeleteDisasterRecoverGroupsResponse>;

/// 修改实例计费模式请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ModifyInstancesChargeTypeRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例计费模式。
    /// PREPAID：预付费，即包年包月
    /// POSTPAID_BY_HOUR：后付费，即按量计费
    /// CDHPAID：CDH付费，即只对CDH计费，不对CDH上的实例计费
    /// SPOTPAID：竞价实例付费
    /// 默认值：PREPAID。
    pub InstanceChargeType: String,
    
    /// 预付费模式，即包年包月相关参数设置。通过该参数可以指定包年包月实例的购买时长、是否设置自动续费等属性。
    /// 若指定实例的付费模式为预付费则该参数必传。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargePrepaid: Option<InstanceChargePrepaid>,
    
    /// 是否同时切换弹性数据云盘计费模式。取值范围：
    /// TRUE：表示切换弹性数据云盘计费模式
    /// FALSE：表示不切换弹性数据云盘计费模式
    /// 默认取值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ModifyPortableDataDisk: Option<bool>,
}

/// 修改实例计费模式响应
#[derive(Debug, Deserialize)]
pub struct ModifyInstancesChargeTypeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 修改实例计费模式响应类型
pub type ModifyInstancesChargeTypeResponseType = ApiResponse<ModifyInstancesChargeTypeResponse>;

/// 修改实例续费标识请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ModifyInstancesRenewFlagRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求允许操作的实例数量上限是100。
    pub InstanceIds: Vec<String>,
    
    /// 自动续费标识。取值范围：
    /// NOTIFY_AND_AUTO_RENEW：通知过期且自动续费
    /// NOTIFY_AND_MANUAL_RENEW：通知过期不自动续费
    /// DISABLE_NOTIFY_AND_MANUAL_RENEW：不通知过期不自动续费
    pub RenewFlag: String,
}

/// 修改实例续费标识响应
#[derive(Debug, Deserialize)]
pub struct ModifyInstancesRenewFlagResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 修改实例续费标识响应类型
pub type ModifyInstancesRenewFlagResponseType = ApiResponse<ModifyInstancesRenewFlagResponse>;

/// 修改实例的硬盘介质请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ModifyInstanceDiskTypeRequest {
    /// 待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。
    pub InstanceId: String,
    
    /// 实例数据盘配置信息，只需要指定要转换的目标类型，标识号不允许修改。
    pub DataDisks: Vec<InstanceDiskType>,
    
    /// 实例系统盘配置信息，只需要指定要转换的目标类型，标识号不允许修改。
    pub SystemDisk: InstanceDiskType,
}

/// 实例磁盘类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceDiskType {
    /// 数据盘或系统盘ID
    pub DiskId: String,
    
    /// 磁盘类型。目标磁盘类型。取值范围：
    /// CLOUD_BASIC：表示普通云硬盘
    /// CLOUD_PREMIUM：表示高性能云硬盘
    /// CLOUD_SSD：表示SSD云硬盘
    /// CLOUD_HSSD：表示增强型SSD云硬盘
    pub DiskType: String,
}

/// 修改实例的硬盘介质响应
#[derive(Debug, Deserialize)]
pub struct ModifyInstanceDiskTypeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 修改实例的硬盘介质响应类型
pub type ModifyInstanceDiskTypeResponseType = ApiResponse<ModifyInstanceDiskTypeResponse>;

/// 进入救援模式请求参数
#[derive(Debug, Clone, Serialize)]
pub struct EnterRescueModeRequest {
    /// 待进入救援模式的实例ID。
    pub InstanceId: String,
}

/// 进入救援模式响应
#[derive(Debug, Deserialize)]
pub struct EnterRescueModeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 进入救援模式响应类型
pub type EnterRescueModeResponseType = ApiResponse<EnterRescueModeResponse>;

/// 退出救援模式请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ExitRescueModeRequest {
    /// 待退出救援模式的实例ID。
    pub InstanceId: String,
}

/// 退出救援模式响应
#[derive(Debug, Deserialize)]
pub struct ExitRescueModeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 退出救援模式响应类型
pub type ExitRescueModeResponseType = ApiResponse<ExitRescueModeResponse>;

/// 查询可用区的机型配置信息的过滤条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Filter {
    /// 过滤条件的名称
    pub Name: String,
    
    /// 过滤条件的值
    pub Values: Vec<String>,
}

/// 获取可用区机型配置信息请求
#[derive(Debug, Clone, Serialize)]
pub struct DescribeZoneInstanceConfigInfosRequest {
    /// 过滤条件，可支持的过滤条件如下：
    /// - zone: 按照可用区过滤，如 ap-guangzhou-1
    /// - instance-family: 按照实例机型系列过滤，如 S1、I1、M1等
    /// - instance-type: 按照实例机型过滤
    /// - instance-charge-type: 按照实例计费模式过滤
    /// - sort-keys: 按关键字排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Filters: Option<Vec<Filter>>,
}

/// 机型配置价格信息
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceTypePrice {
    /// 描述了按带宽计费的价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPrice: Option<f64>,
    
    /// 描述了按流量计费的价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPriceDiscount: Option<f64>,
    
    /// 步进价格阶梯1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPriceSecondStep: Option<f64>,
    
    /// 步进价格阶梯2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPriceThirdStep: Option<f64>,
    
    /// 步进价格折后阶梯1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPriceDiscountSecondStep: Option<f64>,
    
    /// 步进价格折后阶梯2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnitPriceDiscountThirdStep: Option<f64>,
    
    /// 原始价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OriginalPrice: Option<f64>,
    
    /// 折扣价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountPrice: Option<f64>,
    
    /// 折扣
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Discount: Option<f64>,
    
    /// 计费单位：HOUR(小时)，MONTH(月)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ChargeUnit: Option<String>,
    
    /// 一年原始价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OriginalPriceOneYear: Option<f64>,
    
    /// 一年折扣价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountPriceOneYear: Option<f64>,
    
    /// 一年折扣
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountOneYear: Option<f64>,
    
    /// 三年原始价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OriginalPriceThreeYear: Option<f64>,
    
    /// 三年折扣价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountPriceThreeYear: Option<f64>,
    
    /// 三年折扣
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountThreeYear: Option<f64>,
    
    /// 五年原始价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub OriginalPriceFiveYear: Option<f64>,
    
    /// 五年折扣价格
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountPriceFiveYear: Option<f64>,
    
    /// 五年折扣
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiscountFiveYear: Option<f64>,
}

/// 外部数据结构
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceTypeExternals {
    /// 释放地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ReleaseAddress: Option<bool>,
    
    /// 不支持的特性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UnsupportedFeatures: Option<Vec<String>>,
    
    /// 存储接口类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub StorageBlockAttr: Option<StorageBlock>,
    
    /// CpuType对应的PlatformId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CpuPlatformId: Option<i32>,
}

/// 存储块属性
#[derive(Debug, Clone, Deserialize)]
pub struct StorageBlock {
    /// 本地存储类型，值为：LOCAL_BASIC、LOCAL_SSD、LOCAL_NVME、LOCAL_HDD、CLOUD_BASIC、CLOUD_PREMIUM、CLOUD_SSD 等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Type: Option<String>,
    
    /// 本地存储的最小大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub MinSize: Option<i32>,
    
    /// 本地存储的最大大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub MaxSize: Option<i32>,
}

/// 本地磁盘规格
#[derive(Debug, Clone, Deserialize)]
pub struct LocalDiskType {
    /// 磁盘类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Type: Option<String>,
    
    /// 磁盘描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PartitionType: Option<String>,
    
    /// 磁盘大小，单位GB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub MinSize: Option<i32>,
    
    /// 磁盘大小，单位GB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub MaxSize: Option<i32>,
    
    /// 磁盘是否必选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Required: Option<String>,
}

/// 机型配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceTypeQuotaItem {
    /// 可用区。
    pub Zone: String,
    
    /// 实例机型。
    pub InstanceType: String,
    
    /// 实例计费模式。取值范围： PREPAID：表示预付费，即包年包月 POSTPAID_BY_HOUR：表示后付费，即按量计费 CDHPAID：表示CDH付费，即只对CDH计费，不对CDH上的实例计费。 SPOTPAID：表示竞价实例付费 CDCPAID：表示专用集群付费
    pub InstanceChargeType: String,
    
    /// 网卡类型。1：表示实例位于基础网络 0：表示实例位于VPC中。
    pub NetworkCard: i32,
    
    /// 扩展属性。
    pub Externals: InstanceTypeExternals,
    
    /// 实例的CPU核数，单位：核。
    pub Cpu: i32,
    
    /// 实例内存容量，单位：GB。
    pub Memory: f64,
    
    /// 实例机型系列。
    pub InstanceFamily: String,
    
    /// 机型名称。
    pub TypeName: String,
    
    /// 本地磁盘规格列表。当该参数返回为空值时，表示当前情况下无法创建本地盘。
    pub LocalDiskTypeList: Vec<LocalDiskType>,
    
    /// 实例是否售卖。取值范围： SELL：表示实例可购买 SOLD_OUT：表示实例已售罄。
    pub Status: String,
    
    /// 实例的售卖状态。取值范围： AVAILABLE：运营中 UNAVAILABLE：不可用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub StatusCategory: Option<String>,
    
    /// 机型售罄原因。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SoldOutReason: Option<String>,
    
    /// 实例的价格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Price: Option<InstanceTypePrice>,
    
    /// 机型处理器类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub CpuType: Option<String>,
    
    /// 机型处理器主频。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Frequency: Option<String>,
    
    /// 实例挂载的块存储设备数量上限。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub StorageBlockAmount: Option<i32>,
    
    /// 实例网络性能的上限。 单位：Gbps
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceBandwidth: Option<f64>,
    
    /// 网络收发包能力上限。 单位：万PPS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstancePps: Option<f64>,
    
    /// GPU数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Gpu: Option<f64>,
    
    /// GPU类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub GpuCount: Option<f64>,
    
    /// FPGA数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Fpga: Option<f64>,
    
    /// 备注信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Remark: Option<String>,
}

/// 获取可用区机型配置信息响应
#[derive(Debug, Deserialize)]
pub struct DescribeZoneInstanceConfigInfosResponse {
    /// 可用区机型配置列表。
    pub InstanceTypeQuotaSet: Vec<InstanceTypeQuotaItem>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 获取可用区机型配置信息响应类型
pub type DescribeZoneInstanceConfigInfosResponseType = ApiResponse<DescribeZoneInstanceConfigInfosResponse>;

/// 批量获取指定实例属性请求参数
#[derive(Debug, Clone, Serialize)]
pub struct DescribeInstancesAttributeRequest {
    /// 一个或多个待操作的实例ID。可通过DescribeInstances接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
}

/// 批量获取指定实例属性响应
#[derive(Debug, Deserialize)]
pub struct DescribeInstancesAttributeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 批量获取指定实例属性响应类型
pub type DescribeInstancesAttributeResponseType = ApiResponse<DescribeInstancesAttributeResponse>;

/// 实例配置相关服务
pub struct InstanceConfigService<'a> {
    client: &'a TencentCloudClient,
}

impl<'a> InstanceConfigService<'a> {
    /// 创建新的实例配置服务
    pub fn new(client: &'a TencentCloudClient) -> Self {
        Self { client }
    }

    /// 查询用户配额详情
    /// 
    /// 本接口(DescribeAccountQuota)用于查询用户配额详情。
    pub async fn describe_account_quota(&self, request: &DescribeAccountQuotaRequest, region: &str) -> Result<DescribeAccountQuotaResponseType> {
        self.client.request(
            "DescribeAccountQuota", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 创建定时任务
    /// 
    /// 本接口(CreateDisasterRecoverGroup)用于创建分散置放群组。
    pub async fn create_disaster_recover_group(&self, request: &CreateDisasterRecoverGroupRequest, region: &str) -> Result<CreateDisasterRecoverGroupResponseType> {
        self.client.request(
            "CreateDisasterRecoverGroup", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 调整实例配置
    /// 
    /// 本接口(ResizeInstances)用于调整实例的配置。
    /// 
    /// - 只支持包年包月实例和按量计费实例
    /// - 允许升级实例的CPU、内存
    /// - 不同机型的增强型数据盘型号不同，所以总共有存储型、计算型等类型。调整配置的时候需要同类型对其调整。
    pub async fn resize_instances(&self, request: &ResizeInstancesRequest, region: &str) -> Result<ResizeInstancesResponseType> {
        self.client.request(
            "ResizeInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 修改实例所属项目
    /// 
    /// 本接口(ModifyInstancesProject)用于修改实例所属项目。
    /// 
    /// - 项目为一个虚拟概念，用户可以在一个账户下面建立多个项目，每个项目中管理不同的资源。
    /// - 项目管理提供了资源管理与授权的功能，可以实现资源的集中管理与监控。
    /// - 该接口可以批量修改实例所属项目。
    pub async fn modify_instances_project(&self, request: &ModifyInstancesProjectRequest, region: &str) -> Result<ModifyInstancesProjectResponseType> {
        self.client.request(
            "ModifyInstancesProject", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 调整实例分散置放群组
    /// 
    /// 本接口(ModifyDisasterRecoverGroupAttribute)用于修改分散置放群组属性。
    pub async fn modify_disaster_recover_group_attribute(&self, request: &ModifyDisasterRecoverGroupAttributeRequest, region: &str) -> Result<ModifyDisasterRecoverGroupAttributeResponseType> {
        self.client.request(
            "ModifyDisasterRecoverGroupAttribute", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 删除定时任务
    /// 
    /// 本接口(DeleteDisasterRecoverGroups)用于删除分散置放群组。
    /// 
    /// - 只有空的置放群组才能被删除，非空的群组需要先销毁组内所有云服务器，才能执行删除操作。
    pub async fn delete_disaster_recover_groups(&self, request: &DeleteDisasterRecoverGroupsRequest, region: &str) -> Result<DeleteDisasterRecoverGroupsResponseType> {
        self.client.request(
            "DeleteDisasterRecoverGroups", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 修改实例计费模式
    /// 
    /// 本接口(ModifyInstancesChargeType)用于将实例的计费模式从按量计费转换为包年包月，或者从包年包月转换为按量计费。
    /// 
    /// - 只支持将按量计费实例转换为包年包月实例
    /// - 只支持将包年包月实例转换为按量计费实例
    /// - 不支持批量操作
    pub async fn modify_instances_charge_type(&self, request: &ModifyInstancesChargeTypeRequest, region: &str) -> Result<ModifyInstancesChargeTypeResponseType> {
        self.client.request(
            "ModifyInstancesChargeType", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 修改实例续费标识
    /// 
    /// 本接口(ModifyInstancesRenewFlag)用于修改包年包月实例续费标识。
    pub async fn modify_instances_renew_flag(&self, request: &ModifyInstancesRenewFlagRequest, region: &str) -> Result<ModifyInstancesRenewFlagResponseType> {
        self.client.request(
            "ModifyInstancesRenewFlag", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 修改实例的硬盘介质
    /// 
    /// 本接口(ModifyInstanceDiskType)用于修改实例的硬盘介质类型。
    pub async fn modify_instance_disk_type(&self, request: &ModifyInstanceDiskTypeRequest, region: &str) -> Result<ModifyInstanceDiskTypeResponseType> {
        self.client.request(
            "ModifyInstanceDiskType", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 进入救援模式
    /// 
    /// 本接口(EnterRescueMode)用于进入救援模式。
    /// 
    /// - 兼容了Linux和Windows操作系统
    pub async fn enter_rescue_mode(&self, request: &EnterRescueModeRequest, region: &str) -> Result<EnterRescueModeResponseType> {
        self.client.request(
            "EnterRescueMode", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 退出救援模式
    /// 
    /// 本接口(ExitRescueMode)用于退出救援模式。
    pub async fn exit_rescue_mode(&self, request: &ExitRescueModeRequest, region: &str) -> Result<ExitRescueModeResponseType> {
        self.client.request(
            "ExitRescueMode", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 获取可用区的机型配置信息
    /// 
    /// 本接口(DescribeZoneInstanceConfigInfos)用于获取可用区的机型信息。
    pub async fn describe_zone_instance_config_infos(
        &self,
        request: &DescribeZoneInstanceConfigInfosRequest,
        region: &str,
    ) -> Result<DescribeZoneInstanceConfigInfosResponseType> {
        self.client.request(
            "DescribeZoneInstanceConfigInfos", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 批量获取指定实例属性
    /// 
    /// 本接口(DescribeInstancesAttribute)用于获取指定CVM实例的详细信息。
    pub async fn describe_instances_attribute(&self, request: &DescribeInstancesAttributeRequest, region: &str) -> Result<DescribeInstancesAttributeResponseType> {
        self.client.request(
            "DescribeInstancesAttribute", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
} 