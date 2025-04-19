//! 实例相关接口
//! 
//! 包含创建实例、查询实例等接口

use serde::{Deserialize, Serialize};

use crate::client::TencentCloudClient;
use crate::error::Result;

/// 实例计费类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InstanceChargeType {
    /// 预付费，即包年包月
    Prepaid,
    /// 按小时后付费
    PostpaidByHour,
    /// 独享子机（基于专用宿主机创建，宿主机部分的资源不收费）
    Cdhpaid,
    /// 竞价付费
    Spotpaid,
    /// 专用集群付费
    Cdcpaid,
}

/// 预付费模式，即包年包月相关参数设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceChargePrepaid {
    /// 购买实例的时长，单位：月。取值范围：1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 24, 36, 48, 60。
    pub Period: i32,

    /// 自动续费标识。取值范围：
    /// NOTIFY_AND_AUTO_RENEW：通知过期且自动续费
    /// NOTIFY_AND_MANUAL_RENEW：通知过期不自动续费
    /// DISABLE_NOTIFY_AND_MANUAL_RENEW：不通知过期不自动续费
    /// 
    /// 默认取值：NOTIFY_AND_MANUAL_RENEW。若该参数指定为NOTIFY_AND_AUTO_RENEW，在账户余额充足的情况下，实例到期后将按月自动续费。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RenewFlag: Option<String>,
}

/// 描述了实例的位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placement {
    /// 实例所属的可用区名称。该参数可以通过调用 DescribeZones 的返回值中的Zone字段来获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Zone: Option<String>,

    /// 实例所属项目ID。该参数可以通过调用 DescribeProjects 的返回值中的 projectId 字段来获取。
    /// 不填为默认项目。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ProjectId: Option<i32>,

    /// 实例所属的专用宿主机ID列表，仅用于入参。如果您有购买专用宿主机并且指定了该参数，则您购买的实例就会随机的部署在这些专用宿主机上。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub HostIds: Option<Vec<String>>,

    /// 指定母机机型，默认值为：S5
    /// 
    /// 全局母机：S1，S2，S3，S4，S5，S6，S4.LARGE16，S4.LARGE32，S4.LARGE48
    /// FPGA母机：SF.V10001
    /// 
    /// 各个地域的主机类型一般情况下不同，详情参考控制台。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub HostIps: Option<Vec<String>>,

    /// 指定专用集群ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DedicatedClusterId: Option<String>,
}

/// 描述了操作系统所在块设备即系统盘的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemDisk {
    /// 系统盘类型。系统盘类型限制详见存储概述。取值范围：
    /// LOCAL_BASIC：本地硬盘
    /// LOCAL_SSD：本地SSD硬盘
    /// CLOUD_BASIC：普通云硬盘
    /// CLOUD_SSD：SSD云硬盘
    /// CLOUD_PREMIUM：高性能云硬盘
    /// 
    /// 默认取值：当前有库存的硬盘类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiskType: Option<String>,

    /// 系统盘ID。参数diskType取值为LOCAL_BASIC或LOCAL_SSD时，无需指定该参数。
    /// 取值为磁盘ID或者快照ID，如：disk-kdt0sq6m或者snap-m4m5vafo。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiskId: Option<String>,

    /// 系统盘大小，单位：GB。默认值为 50
    /// 取值范围：20-500
    /// 不同系统盘类型的默认值各不同。若使用cloud盘，默认值为50。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiskSize: Option<i32>,
}

/// 描述了数据盘的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataDisk {
    /// 数据盘类型。数据盘类型限制详见存储概述。取值范围：
    /// LOCAL_BASIC：本地硬盘
    /// LOCAL_SSD：本地SSD硬盘
    /// LOCAL_NVME：本地NVME硬盘，与InstanceType强相关
    /// LOCAL_PRO：本地HDD硬盘，与InstanceType强相关
    /// CLOUD_BASIC：普通云硬盘
    /// CLOUD_PREMIUM：高性能云硬盘
    /// CLOUD_SSD：SSD云硬盘
    /// CLOUD_HSSD：增强型SSD云硬盘
    /// CLOUD_TSSD：极速型SSD云硬盘
    /// 
    /// 默认取值：LOCAL_BASIC。
    /// 
    /// 该参数对ResizeInstanceDisk接口无效。
    pub DiskType: String,

    /// 数据盘ID。
    /// 可以通过DescribeDisks接口查询已有云硬盘的ID，请参考DescribeDisks接口。
    /// 该参数可以指定数据盘的ID，表示数据盘来源于指定的快照创建。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DiskId: Option<String>,

    /// 数据盘大小，单位：GB。最小调整步长为10G，不同数据盘类型取值范围不同，具体限制详见：存储概述。默认值为0，表示不购买数据盘。更多限制详见产品文档。
    pub DiskSize: i32,

    /// 数据盘是否随子机销毁。取值范围：
    /// TRUE：子机销毁时，销毁数据盘，只支持按小时后付费云盘
    /// FALSE：子机销毁时，保留数据盘
    /// 
    /// 默认取值：TRUE
    /// 
    /// 该参数目前仅用于 RunInstances 接口。
    /// 
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DeleteWithInstance: Option<bool>,
}

/// 描述了VPC相关信息，包括子网，私有IP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualPrivateCloud {
    /// 私有网络ID，形如vpc-xxx。有效的VpcId可通过登录控制台查询；也可以调用接口 DescribeVpcEx ，从接口返回中的unVpcId字段获取。
    /// 若在创建子机时VpcId与SubnetId同时传入SubnetId将自动忽略。通过指定该参数可以指定私有网络的子网IP。
    pub VpcId: String,

    /// 私有网络子网ID，形如subnet-xxx。有效的私有网络子网ID可通过登录控制台查询；也可以调用接口 DescribeSubnets ，从接口返回中的unSubnetId字段获取。
    pub SubnetId: String,

    /// 是否用作公网网关。公网网关只有在实例拥有公网IP以及处于私有网络下时才能正常使用。取值范围：
    /// TRUE：表示用作公网网关
    /// FALSE：表示不用作公网网关
    /// 
    /// 默认取值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub AsVpcGateway: Option<bool>,

    /// 私有网络子网 IP 数组，在创建实例、修改实例vpc属性操作中可使用此参数。
    /// 当前仅批量创建多台实例时支持传入相同子网的多个 IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PrivateIpAddresses: Option<Vec<String>>,
}

/// 描述了实例的公网可访问性，声明了实例的公网使用计费模式，最大带宽等
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InternetAccessible {
    /// 网络计费类型。取值范围：
    /// BANDWIDTH_PREPAID：预付费按带宽结算
    /// TRAFFIC_POSTPAID_BY_HOUR：流量按小时后付费
    /// BANDWIDTH_POSTPAID_BY_HOUR：带宽按小时后付费
    /// BANDWIDTH_PACKAGE：带宽包用户
    /// 
    /// 默认取值：非带宽包用户默认与子机付费类型保持一致。
    pub InternetChargeType: String,

    /// 公网出带宽上限，单位：Mbps。默认值：0Mbps。不同机型带宽上限范围不一致，具体限制详见购买网络带宽。
    pub InternetMaxBandwidthOut: i32,

    /// 是否分配公网IP。取值范围：
    /// TRUE：表示分配公网IP
    /// FALSE：表示不分配公网IP
    /// 
    /// 当公网带宽大于0Mbps时，可自由选择开通与否，默认开通公网IP；当公网带宽为0，则不允许分配公网IP。
    /// 
    /// 该参数仅在RunInstances接口中作为入参使用。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub PublicIpAssigned: Option<bool>,

    /// 带宽包ID。可通过DescribeBandwidthPackages接口返回值中的BandwidthPackageId获取。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub BandwidthPackageId: Option<String>,
}

/// 描述了实例登录相关配置与信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginSettings {
    /// 实例登录密码。不同操作系统类型密码复杂度限制不一样，具体如下：
    /// Linux实例密码必须8到30位，至少包括两项[a-z]，[A-Z]、[0-9] 和 [( ) ` ~ ! @ # $ % ^ & * - + = | { } [ ] : ; ' , . ? / ]中的特殊符号。
    /// Windows实例密码必须12到30位，至少包括三项[a-z]，[A-Z]，[0-9] 和 [( ) ` ~ ! @ # $ % ^ & * - + = | { } [ ] : ; ' , . ? /]中的特殊符号。
    /// 
    /// 若不指定该参数，则由系统随机生成密码，并通过站内信方式通知到用户。
    /// 
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Password: Option<String>,

    /// 密钥ID列表。关联密钥后，就可以通过对应的私钥来访问实例。
    /// 密钥与密码不能同时指定。
    /// 密钥ID列表可以通过接口DescribeKeyPairs获取
    /// 
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub KeyIds: Option<Vec<String>>,

    /// 保持镜像的原始设置。该参数与Password或KeyIds.N不能同时指定。
    /// 只有使用自定义镜像、共享镜像或外部导入镜像创建实例时才能指定该参数为TRUE。
    /// 取值范围：
    /// TRUE：表示保持镜像的登录设置
    /// FALSE：表示不保持镜像的登录设置
    /// 
    /// 默认取值：FALSE。
    /// 
    /// 注意：此字段可能返回 null，表示取不到有效值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub KeepImageLogin: Option<bool>,
}

/// 描述了实例的增强服务启用情况与其设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedService {
    /// 开启云安全服务。若不指定该参数，则默认开启云安全服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SecurityService: Option<RunSecurityServiceEnabled>,

    /// 开启云监控服务。若不指定该参数，则默认开启云监控服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub MonitorService: Option<RunMonitorServiceEnabled>,

    /// 开启云自动化助手服务（TencentCloud Automation Tools，TAT）。若不指定该参数，则默认不开启云自动化助手服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub AutomationService: Option<RunAutomationServiceEnabled>,
}

/// 描述了实例的云监控服务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunMonitorServiceEnabled {
    /// 是否开启云监控服务。取值范围：
    /// TRUE：表示开启云监控服务
    /// FALSE：表示不开启云监控服务
    /// 
    /// 默认取值：TRUE。
    pub Enabled: bool,
}

/// 描述了实例的安全服务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSecurityServiceEnabled {
    /// 是否开启云安全服务。取值范围：
    /// TRUE：表示开启云安全服务
    /// FALSE：表示不开启云安全服务
    /// 
    /// 默认取值：TRUE。
    pub Enabled: bool,
}

/// 描述了实例的TAT服务
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunAutomationServiceEnabled {
    /// 是否开启TAT服务。取值范围：
    /// TRUE：表示开启云自动化助手服务
    /// FALSE：表示不开启云自动化助手服务
    /// 
    /// 默认取值：FALSE。
    pub Enabled: bool,
}

/// 描述了标签对
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagSpecification {
    /// 标签绑定的资源类型，当前支持类型："instance"、"host"、"image"、"snapshot"、"disk"、"vpc"、"subnet"、"security-group"、"region-route-table"
    pub ResourceType: String,

    /// 标签对列表
    pub Tags: Vec<Tag>,
}

/// 标签
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    /// 标签键
    pub Key: String,

    /// 标签值
    pub Value: String,
}

/// 创建实例的请求参数
#[derive(Debug, Clone, Serialize)]
pub struct RunInstancesRequest {
    /// 实例所在的位置。通过该参数可以指定实例所属可用区，所属项目，所属宿主机（在专用宿主机上创建子机时指定）等属性。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub Placement: Option<Placement>,

    /// 指定有效的镜像ID，格式形如img-xxx。镜像类型分为四种：
    /// 公共镜像
    /// 自定义镜像
    /// 共享镜像
    /// 服务市场镜像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ImageId: Option<String>,

    /// 实例计费类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargeType: Option<InstanceChargeType>,

    /// 预付费模式，即包年包月相关参数设置。通过该参数可以指定包年包月实例的购买时长、是否设置自动续费等属性。
    /// 若指定实例的付费模式为预付费则该参数必传。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargePrepaid: Option<InstanceChargePrepaid>,

    /// 实例机型。不同实例机型指定了不同的资源规格。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceType: Option<String>,

    /// 实例系统盘配置信息。若不指定该参数，则按照系统默认值进行分配。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SystemDisk: Option<SystemDisk>,

    /// 实例数据盘配置信息。若不指定该参数，则默认不购买数据盘。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DataDisks: Option<Vec<DataDisk>>,

    /// 私有网络相关信息配置。通过该参数可以指定私有网络的ID，子网ID等信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub VirtualPrivateCloud: Option<VirtualPrivateCloud>,

    /// 公网带宽相关信息设置。若不指定该参数，则默认公网带宽为0Mbps。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InternetAccessible: Option<InternetAccessible>,

    /// 购买实例数量。包年包月实例取值范围：[1，500]，按量计费实例取值范围：[1，500]。默认取值：1。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceCount: Option<i32>,

    /// 实例显示名称。不指定实例显示名称则默认显示'未命名'。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceName: Option<String>,

    /// 实例登录设置。通过该参数可以设置实例的登录方式密码、密钥或保持镜像的原始登录设置。默认情况下会随机生成密码，并以站内信方式知会到用户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub LoginSettings: Option<LoginSettings>,

    /// 实例所属安全组。该参数可以通过调用 DescribeSecurityGroups 的返回值中的sgId字段来获取。若不指定该参数，则绑定默认安全组。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SecurityGroupIds: Option<Vec<String>>,

    /// 增强服务。通过该参数可以指定是否开启云安全、云监控等服务。若不指定该参数，则默认公共镜像开启云监控、云安全服务；自定义镜像与镜像市场镜像默认不开启云监控，云安全服务，而使用镜像里保留的服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub EnhancedService: Option<EnhancedService>,

    /// 用于保证请求幂等性的字符串。该字符串由客户生成，需保证不同请求之间唯一，最大值不超过64个ASCII字符。若不指定该参数，则无法保证请求的幂等性。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ClientToken: Option<String>,

    /// 实例主机名。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub HostName: Option<String>,

    /// 标签描述列表。通过指定该参数可以同时绑定标签到相应的云服务器、云硬盘实例。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub TagSpecification: Option<Vec<TagSpecification>>,

    /// 实例所属项目ID。该参数可以通过调用 DescribeProjects 的返回值中的 projectId 字段来获取。不填为默认项目。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ProjectId: Option<i32>,
}

/// 创建实例的响应
#[derive(Debug, Deserialize)]
pub struct RunInstancesResponse {
    /// 当通过本接口来创建实例时会返回该参数，表示一个或多个实例ID。返回实例ID列表并不代表实例创建成功。
    pub InstanceIdSet: Vec<String>,
    
    /// 唯一请求 ID
    pub RequestId: String,
}

/// API响应通用结构
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// 响应数据
    pub Response: T,
}

/// 创建实例响应类型
pub type RunInstancesResponseType = ApiResponse<RunInstancesResponse>;

/// 退还实例的请求参数
#[derive(Debug, Clone, Serialize)]
pub struct TerminateInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。
    /// 每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,

    /// 释放实例挂载的包年包月数据盘。
    /// true表示销毁实例同时释放包年包月数据盘，false表示只销毁实例。
    /// 默认值：false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ReleasePrepaidDataDisks: Option<bool>,
}

/// 退还实例的响应
#[derive(Debug, Deserialize)]
pub struct TerminateInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 退还实例响应类型
pub type TerminateInstancesResponseType = ApiResponse<TerminateInstancesResponse>;

/// 实例服务
pub struct InstanceService<'a> {
    client: &'a TencentCloudClient,
}

impl<'a> InstanceService<'a> {
    /// 创建新的实例服务
    pub fn new(client: &'a TencentCloudClient) -> Self {
        Self { client }
    }

    /// 创建一个或多个指定配置的实例
    /// 
    /// 本接口(RunInstances)用于创建一个或多个指定配置的实例。
    pub async fn run_instances(&self, request: &RunInstancesRequest, region: &str) -> Result<RunInstancesResponseType> {
        // 使用client发送请求
        self.client.request(
            "RunInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }

    /// 主动退还实例
    /// 
    /// 本接口(TerminateInstances)用于主动退还实例。
    /// 
    /// - 不再使用的实例，可通过本接口主动退还。
    /// - 按量计费的实例通过本接口可直接退还；包年包月实例如符合退还规则，也可通过本接口主动退还。
    /// - 包年包月实例首次调用本接口，实例将被移至回收站，再次调用本接口，实例将被销毁，且不可恢复。按量计费实例调用本接口将被直接销毁。
    pub async fn terminate_instances(&self, request: &TerminateInstancesRequest, region: &str) -> Result<TerminateInstancesResponseType> {
        // 使用client发送请求
        self.client.request(
            "TerminateInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
} 