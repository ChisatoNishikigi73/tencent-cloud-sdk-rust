//! 实例操作相关接口
//! 
//! 包含启动实例、重启实例、关闭实例、重装实例等操作

use serde::{Deserialize, Serialize};

use crate::client::TencentCloudClient;
use crate::error::Result;

/// 启动实例的请求参数
#[derive(Debug, Clone, Serialize)]
pub struct StartInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。
    /// 每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
}

/// 启动实例的响应
#[derive(Debug, Deserialize)]
pub struct StartInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 启动实例响应类型
pub type StartInstancesResponseType = crate::services::cvm::instance::ApiResponse<StartInstancesResponse>;

/// 重启实例的请求参数
#[derive(Debug, Clone, Serialize)]
pub struct RebootInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。
    /// 每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,

    /// 关机类型。取值范围：
    /// SOFT：表示软关机
    /// HARD：表示硬关机
    /// SOFT_FIRST：表示优先软关机，失败再执行硬关机
    /// 
    /// 默认取值：SOFT。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub StopType: Option<String>,

    /// 表示是否在正常重启失败后选择强制重启实例。
    /// true：表示在正常重启失败后进行强制重启
    /// false：表示在正常重启失败后不进行强制重启
    /// 
    /// 默认取值：false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ForceReboot: Option<bool>,
}

/// 重启实例的响应
#[derive(Debug, Deserialize)]
pub struct RebootInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 重启实例响应类型
pub type RebootInstancesResponseType = crate::services::cvm::instance::ApiResponse<RebootInstancesResponse>;

/// 关闭实例的请求参数
#[derive(Debug, Clone, Serialize)]
pub struct StopInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。
    /// 每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,

    /// 实例的关闭模式。取值范围：
    /// SOFT_FIRST：表示在正常关闭失败后进行强制关闭
    /// HARD：直接强制关闭
    /// SOFT：仅软关机
    /// 
    /// 默认取值：SOFT。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub StopType: Option<String>,

    /// 表示是否在正常关闭失败后选择强制关闭实例。
    /// true：表示在正常关闭失败后进行强制关闭
    /// false：表示在正常关闭失败后不进行强制关闭
    /// 
    /// 默认取值：false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ForceStop: Option<bool>,

    /// 按量计费实例关机收费模式。
    /// 取值范围：
    /// KEEP_CHARGING：关机继续收费
    /// STOP_CHARGING：关机停止收费
    /// 
    /// 默认取值：KEEP_CHARGING。
    /// 该参数只针对部分按量计费云硬盘实例生效。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub StoppedMode: Option<String>,
}

/// 关闭实例的响应
#[derive(Debug, Deserialize)]
pub struct StopInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 关闭实例响应类型
pub type StopInstancesResponseType = crate::services::cvm::instance::ApiResponse<StopInstancesResponse>;

/// 重置实例密码请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ResetInstancesPasswordRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例登录密码。不同操作系统类型密码复杂度限制不一样:
    /// Linux实例密码必须8到30位，至少包括两项[a-z]，[A-Z]、[0-9] 和 [( ) ` ~ ! @ # $ % ^ & * - + = | { } [ ] : ; ' , . ? / ]中的特殊符号。
    /// Windows实例密码必须12到30位，至少包括三项[a-z]，[A-Z]，[0-9] 和 [( ) ` ~ ! @ # $ % ^ & * - + = | { } [ ] : ; ' , . ? /]中的特殊符号。
    pub Password: String,
    
    /// 是否对运行中的实例选择强制关机。建议对运行中的实例先手动关机，然后再重置密码。取值范围：
    /// TRUE：表示在正常关机失败后进行强制关机
    /// FALSE：表示在正常关机失败后不进行强制关机
    /// 
    /// 默认取值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ForceStop: Option<bool>,
    
    /// 待重置密码的实例操作系统的用户名。不得超过64个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub UserName: Option<String>,
}

/// 重置实例密码响应
#[derive(Debug, Deserialize)]
pub struct ResetInstancesPasswordResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 重置实例密码响应类型
pub type ResetInstancesPasswordResponseType = crate::services::cvm::instance::ApiResponse<ResetInstancesPasswordResponse>;

/// 修改实例属性请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ModifyInstancesAttributeRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 实例名称。可任意命名，但不得超过60个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceName: Option<String>,
    
    /// 实例所属安全组。该参数可以通过调用 DescribeSecurityGroups 的返回值中的sgId字段来获取。若不指定该参数，则默认不修改安全组。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub SecurityGroups: Option<Vec<String>>,
    
    /// 是否开启实例销毁保护，取值范围：
    /// TRUE：表示开启实例保护，不允许通过api接口删除实例
    /// FALSE：表示关闭实例保护，允许通过api接口删除实例
    /// 
    /// 默认取值：FALSE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub DisableApiTermination: Option<bool>,
    
    /// 实例关机模式。取值范围：
    /// SOFT_FIRST：表示在正常关机失败后进行强制关机
    /// HARD：表示直接强制关机
    /// SOFT：表示仅软关机
    /// 
    /// 默认取值：SOFT。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ShutdownBehavior: Option<String>,
    
    /// 实例计费模式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub InstanceChargeType: Option<String>,
}

/// 修改实例属性响应
#[derive(Debug, Deserialize)]
pub struct ModifyInstancesAttributeResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 修改实例属性响应类型
pub type ModifyInstancesAttributeResponseType = crate::services::cvm::instance::ApiResponse<ModifyInstancesAttributeResponse>;

/// 续费实例请求参数
#[derive(Debug, Clone, Serialize)]
pub struct RenewInstancesRequest {
    /// 一个或多个待操作的实例ID。可通过 DescribeInstances 接口返回值中的InstanceId获取。每次请求批量实例的上限为100。
    pub InstanceIds: Vec<String>,
    
    /// 预付费模式，即包年包月相关参数设置。通过该参数可以指定包年包月实例的续费时长、是否设置自动续费等属性。
    pub InstanceChargePrepaid: crate::services::cvm::instance::InstanceChargePrepaid,
    
    /// 是否续费弹性数据盘。取值范围：
    /// TRUE：表示续费包年包月实例同时续费其挂载的弹性数据盘
    /// FALSE：表示续费包年包月实例同时不再续费其挂载的弹性数据盘
    /// 
    /// 默认取值：TRUE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub RenewPortableDataDisk: Option<bool>,
}

/// 续费实例响应
#[derive(Debug, Deserialize)]
pub struct RenewInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 续费实例响应类型
pub type RenewInstancesResponseType = crate::services::cvm::instance::ApiResponse<RenewInstancesResponse>;

/// 重装实例请求参数
#[derive(Debug, Clone, Serialize)]
pub struct ResetInstanceRequest {
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
    pub SystemDisk: Option<crate::services::cvm::instance::SystemDisk>,
    
    /// 实例登录设置。通过该参数可以设置实例的登录方式密码、密钥或保持镜像的原始登录设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub LoginSettings: Option<crate::services::cvm::instance::LoginSettings>,
    
    /// 增强服务。通过该参数可以指定是否开启云安全、云监控等服务。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub EnhancedService: Option<crate::services::cvm::instance::EnhancedService>,
}

/// 重装实例响应
#[derive(Debug, Deserialize)]
pub struct ResetInstanceResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 重装实例响应类型
pub type ResetInstanceResponseType = crate::services::cvm::instance::ApiResponse<ResetInstanceResponse>;

/// 退还实例请求参数
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

/// 退还实例响应
#[derive(Debug, Deserialize)]
pub struct TerminateInstancesResponse {
    /// 唯一请求 ID
    pub RequestId: String,
}

/// 退还实例响应类型
pub type TerminateInstancesResponseType = crate::services::cvm::instance::ApiResponse<TerminateInstancesResponse>;

/// 实例操作相关服务
pub struct InstanceOperationService<'a> {
    client: &'a TencentCloudClient,
}

impl<'a> InstanceOperationService<'a> {
    /// 创建新的实例操作服务
    pub fn new(client: &'a TencentCloudClient) -> Self {
        Self { client }
    }

    /// 启动实例
    /// 
    /// 本接口 (StartInstances) 用于启动一个或多个实例。
    /// 
    /// - 只有状态为STOPPED的实例才可以进行此操作。
    /// - 接口调用成功时，实例会进入STARTING状态；启动实例成功时，实例会进入RUNNING状态。
    /// - 本接口为异步接口，启动实例请求发送成功后会返回一个RequestId，此时操作并未立即完成。
    pub async fn start_instances(&self, request: &StartInstancesRequest, region: &str) -> Result<StartInstancesResponseType> {
        self.client.request(
            "StartInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }

    /// 重启实例
    /// 
    /// 本接口 (RebootInstances) 用于重启实例。
    /// 
    /// - 只有状态为RUNNING的实例才可以进行此操作。
    /// - 接口调用成功时，实例会进入REBOOTING状态；重启实例成功时，实例会进入RUNNING状态。
    /// - 支持强制重启，强制重启可能会导致数据丢失或文件系统损坏，请仅在服务器不能正常重启时使用。
    pub async fn reboot_instances(&self, request: &RebootInstancesRequest, region: &str) -> Result<RebootInstancesResponseType> {
        self.client.request(
            "RebootInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }

    /// 关闭实例
    /// 
    /// 本接口 (StopInstances) 用于关闭一个或多个实例。
    /// 
    /// - 只有状态为RUNNING的实例才可以进行此操作。
    /// - 接口调用成功时，实例会进入STOPPING状态；关闭实例成功时，实例会进入STOPPED状态。
    /// - 支持强制关闭，强制关闭可能会导致数据丢失或文件系统损坏，请仅在服务器不能正常关机时使用。
    /// - 本接口为异步接口，关闭实例请求发送成功后会返回一个RequestId，此时操作并未立即完成。
    pub async fn stop_instances(&self, request: &StopInstancesRequest, region: &str) -> Result<StopInstancesResponseType> {
        self.client.request(
            "StopInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 重置实例密码
    /// 
    /// 本接口 (ResetInstancesPassword) 用于将实例操作系统的密码重置为用户指定的密码。
    /// 
    /// - 只有状态为RUNNING或STOPPED的实例才可以进行此操作。
    /// - 批量操作的每个实例的重置密码结果可能不同，具体操作结果可以通过调用 DescribeInstances 接口查询。
    /// - 本接口为异步接口，重置密码请求发送成功后会返回一个RequestId，此时操作并未立即完成。
    pub async fn reset_instances_password(&self, request: &ResetInstancesPasswordRequest, region: &str) -> Result<ResetInstancesPasswordResponseType> {
        self.client.request(
            "ResetInstancesPassword", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 修改实例的属性
    /// 
    /// 本接口 (ModifyInstancesAttribute) 用于修改实例的属性（目前只支持修改实例的名称）。
    /// 
    /// - 批量操作的每个实例的修改属性结果可能不同，具体操作结果可以通过调用 DescribeInstances 接口查询。
    pub async fn modify_instances_attribute(&self, request: &ModifyInstancesAttributeRequest, region: &str) -> Result<ModifyInstancesAttributeResponseType> {
        self.client.request(
            "ModifyInstancesAttribute", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 续费实例
    /// 
    /// 本接口 (RenewInstances) 用于续费包年包月实例。
    /// 
    /// - 只支持操作包年包月实例。
    /// - 批量续费实例的续费时间将以所有实例中最短的剩余时间为准。例如，三个实例分别有1个月、2个月、3个月的剩余时间，续费2个月，则1个月的实例续费后剩余时间为3个月，2个月的实例续费后剩余时间为4个月，3个月的实例续费后剩余时间为5个月。
    pub async fn renew_instances(&self, request: &RenewInstancesRequest, region: &str) -> Result<RenewInstancesResponseType> {
        self.client.request(
            "RenewInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 重装实例
    /// 
    /// 本接口 (ResetInstance) 用于重装指定实例上的操作系统。
    /// 
    /// - 如果指定了ImageId参数，则使用指定的镜像重装；否则按照系统默认值进行重装。
    /// - 系统盘将会被格式化，并重置为指定的操作系统，其中数据盘的数据将保留不做处理。
    /// - 只有状态为RUNNING或者STOPPED的实例才可以进行此操作。
    pub async fn reset_instance(&self, request: &ResetInstanceRequest, region: &str) -> Result<ResetInstanceResponseType> {
        self.client.request(
            "ResetInstance", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
    
    /// 退还实例
    /// 
    /// 本接口(TerminateInstances)用于主动退还实例。
    /// 
    /// - 不再使用的实例，可通过本接口主动退还。
    /// - 按量计费的实例通过本接口可直接退还；包年包月实例如符合退还规则，也可通过本接口主动退还。
    /// - 包年包月实例首次调用本接口，实例将被移至回收站，再次调用本接口，实例将被销毁，且不可恢复。按量计费实例调用本接口将被直接销毁。
    pub async fn terminate_instances(&self, request: &TerminateInstancesRequest, region: &str) -> Result<TerminateInstancesResponseType> {
        self.client.request(
            "TerminateInstances", 
            request, 
            "cvm", 
            "2017-03-12", 
            Some(region)
        ).await
    }
} 