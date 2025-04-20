use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance::{
        InstanceService, RunInstancesRequest, Placement, LoginSettings,
        SystemDisk, InternetAccessible, EnhancedService, RunSecurityServiceEnabled,
        RunMonitorServiceEnabled, InstanceChargeType
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取密钥
    let secret_id = std::env::var("TENCENTCLOUD_SECRET_ID")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_ID");
    let secret_key = std::env::var("TENCENTCLOUD_SECRET_KEY")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_KEY");

    // 创建客户端
    let client = TencentCloudClient::new(secret_id, secret_key);
    
    // 创建实例服务
    let instance_service = InstanceService::new(&client);
    
    // 设置区域
    let region = "ap-guangzhou";
    
    // 创建实例请求参数
    let request = RunInstancesRequest {
        // 设置实例位置（广州六区）
        Placement: Some(Placement {
            Zone: Some("ap-guangzhou-6".to_string()),
            ProjectId: None,
            HostIds: None,
            HostIps: None,
            DedicatedClusterId: None,
        }),
        
        // 指定镜像ID - TencentOS
        ImageId: Some("img-6n21msk1".to_string()),
        
        // 实例计费类型 - 竞价实例
        InstanceChargeType: Some(InstanceChargeType::Spotpaid),
        
        // 实例配置 - S5.MEDIUM2
        InstanceType: Some("S5.MEDIUM2".to_string()),
        
        // 系统盘 - 使用CLOUD_BSSD
        SystemDisk: Some(SystemDisk {
            DiskType: Some("CLOUD_BSSD".to_string()),
            DiskId: None,
            DiskSize: Some(20),
        }),
        
        // 不指定VPC配置，使用默认VPC和子网
        VirtualPrivateCloud: None,
        
        // 使用默认安全组
        SecurityGroupIds: None,
        
        // 实例数量
        InstanceCount: Some(1),
        
        // 实例名称
        InstanceName: Some("test".to_string()),
        
        // 登录设置
        LoginSettings: Some(LoginSettings {
            Password: Some("Test@123456789".to_string()),
            KeyIds: None,
            KeepImageLogin: None,
        }),
        
        // 公网带宽 - 不需要公网IP
        InternetAccessible: Some(InternetAccessible {
            InternetChargeType: "TRAFFIC_POSTPAID_BY_HOUR".to_string(),
            InternetMaxBandwidthOut: 1,
            PublicIpAssigned: Some(true),
            BandwidthPackageId: None,
        }),
        
        // 增强服务
        EnhancedService: Some(EnhancedService {
            SecurityService: Some(RunSecurityServiceEnabled {
                Enabled: true,
            }),
            MonitorService: Some(RunMonitorServiceEnabled {
                Enabled: true,
            }),
            AutomationService: None,
        }),
        
        // 其他参数
        DataDisks: None,
        ClientToken: None,
        HostName: None,
        TagSpecification: None,
        ProjectId: None,
        InstanceChargePrepaid: None,
    };
    
    // 发送创建请求
    println!("正在创建竞价实例...");
    match instance_service.run_instances(&request, region).await {
        Ok(response) => {
            println!("实例创建请求已提交，实例ID列表:");
            for id in response.Response.InstanceIdSet {
                println!("- {}", id);
            }
            println!("\n注意：返回实例ID列表并不代表实例创建成功，请通过DescribeInstances接口查询实例状态");
        },
        Err(err) => {
            println!("创建实例失败: {}", err);
        }
    }
    
    Ok(())
} 