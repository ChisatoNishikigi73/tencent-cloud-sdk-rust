# 腾讯云API SDK (Rust)

这是一个非官方的腾讯云API Rust SDK，提供了与腾讯云API进行交互的Rust实现。

## 目录

- [安装和环境配置](#安装和环境配置)
- [已实现功能](#已实现功能)
- [使用方法](#使用方法)
  - [查询地域和可用区](#1-查询地域和可用区)
  - [创建实例](#2-创建实例)
  - [退还实例](#3-退还实例)
- [许可证](#许可证)

## 安装和环境配置

1. 在Cargo.toml中添加依赖
2. 设置环境变量

```bash
export TENCENTCLOUD_SECRET_ID="你的腾讯云SecretId"
export TENCENTCLOUD_SECRET_KEY="你的腾讯云SecretKey"
```

## 已实现功能

### 通用功能

- ✅ 腾讯云API签名V3算法完整实现
- ✅ HTTP客户端封装
- ✅ 统一的错误处理

### 地域与可用区管理

- ✅ 查询地域列表 (DescribeRegions)
- ✅ 查询可用区列表 (DescribeZones)

### 云服务器实例管理

- ✅ 创建实例 (RunInstances)
  - 支持按量计费和竞价实例
  - 支持自定义配置（实例类型、系统盘等）
  - 支持网络配置和安全组配置
  
- ✅ 退还实例 (TerminateInstances)
  - 支持批量退还
  - 支持按量计费和包年包月实例

## 使用方法

### 1. 查询地域和可用区

```rust
use tencent_cloud_sdk::{TencentCloudClient, services::region::RegionService};

// 创建客户端
let client = TencentCloudClient::new(secret_id, secret_key);

// 创建地域服务
let region_service = RegionService::new(&client);

// 查询地域列表 - 无需参数
let regions = region_service.describe_regions().await?;

// 查询可用区列表 - 需要地域参数
let zones = region_service.describe_zones("ap-guangzhou").await?;
```

### 2. 创建实例

```rust
use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance::{
        InstanceService, RunInstancesRequest, Placement, LoginSettings,
        SystemDisk, InstanceChargeType
    }
};

// 创建客户端和服务
let client = TencentCloudClient::new(secret_id, secret_key);
let instance_service = InstanceService::new(&client);

// 创建实例请求参数
let request = RunInstancesRequest {
    // 必填参数
    Placement: Some(Placement {
        Zone: Some("ap-guangzhou-6".to_string()),
        // 其他字段可选
        ProjectId: None,
        HostIds: None,
        HostIps: None,
        DedicatedClusterId: None,
    }),
    // Image: TencentOS
    ImageId: Some("img-6n21msk1".to_string()),
    
    // 可选参数
    InstanceChargeType: Some(InstanceChargeType::Spotpaid), // 竞价实例
    InstanceType: Some("S5.MEDIUM2".to_string()),
    SystemDisk: Some(SystemDisk {
        DiskType: Some("CLOUD_BSSD".to_string()),
        DiskSize: Some(20),
        DiskId: None,
    }),
    
    // 其他参数，按需设置
    InstanceCount: Some(1),
    LoginSettings: Some(LoginSettings {
        Password: Some("Password123".to_string()),
        KeyIds: None,
        KeepImageLogin: None,
    }),
    
    // 默认为None的参数可省略
    VirtualPrivateCloud: None,
    SecurityGroupIds: None,
    InstanceName: Some("test-instance".to_string()),
    // ... 更多参数
};

// 发送请求
let response = instance_service.run_instances(&request, "ap-guangzhou").await?;
```

### 3. 退还实例

```rust
use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance::{InstanceService, TerminateInstancesRequest}
};

// 创建客户端和服务
let client = TencentCloudClient::new(secret_id, secret_key);
let instance_service = InstanceService::new(&client);

// 创建退还实例请求
let request = TerminateInstancesRequest {
    // 必填参数 - 实例ID列表
    InstanceIds: vec!["ins-xxxxxxxx".to_string()],
    
    // 可选参数
    ReleasePrepaidDataDisks: Some(false), // 是否释放包年包月数据盘
};

// 发送请求
let response = instance_service.terminate_instances(&request, "ap-guangzhou").await?;
```

## 许可证

[MIT许可证](https://choosealicense.com/licenses/mit)
