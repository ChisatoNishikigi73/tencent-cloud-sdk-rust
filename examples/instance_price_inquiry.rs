use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance_price::{
        InstancePriceService, InquiryPriceRunInstancesRequest
    },
    services::cvm::instance::{
        Placement, SystemDisk, DataDisk, InternetAccessible, 
        InstanceChargeType, InstanceChargePrepaid, LoginSettings,
        EnhancedService, RunSecurityServiceEnabled, RunMonitorServiceEnabled
    }
};
use std::env;
use std::collections::HashMap;

// 默认参数值
const DEFAULT_REGION: &str = "ap-guangzhou";
const DEFAULT_ZONE: &str = "ap-guangzhou-6";
const DEFAULT_IMAGE_ID: &str = "img-eb30mz89"; // TencentOS Server 3.2
const DEFAULT_INSTANCE_TYPE: &str = "S5.MEDIUM2"; // 2核4G
const DEFAULT_SYSTEM_DISK_TYPE: &str = "CLOUD_PREMIUM"; // 高性能云硬盘
const DEFAULT_SYSTEM_DISK_SIZE: i32 = 50;
const DEFAULT_DATA_DISK_TYPE: &str = "CLOUD_PREMIUM";
const DEFAULT_DATA_DISK_SIZE: i32 = 100;
const DEFAULT_BANDWIDTH: i32 = 10;
const DEFAULT_CHARGE_TYPE: &str = "postpaid"; // 按量计费
const DEFAULT_PERIOD: i32 = 1; // 1个月
const DEFAULT_INSTANCE_COUNT: i32 = 1;
const DEFAULT_PASSWORD: &str = "YourPassword123";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取密钥
    let secret_id = env::var("TENCENTCLOUD_SECRET_ID")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_ID");
    let secret_key = env::var("TENCENTCLOUD_SECRET_KEY")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_KEY");

    // 创建客户端
    let client = TencentCloudClient::new(secret_id, secret_key);
    
    // 创建实例价格服务
    let instance_price_service = InstancePriceService::new(&client);
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 解析命令行参数到HashMap
    let params = parse_args(&args);

    println!("params: {:?}", params);
    
    // 如果指定了帮助参数，显示帮助信息并退出
    if params.contains_key("help") {
        print_usage();
        return Ok(());
    }
    
    // 从参数中获取值，如果没有则使用默认值
    let region = params.get("region").cloned().unwrap_or_else(|| DEFAULT_REGION.to_string());
    let zone = params.get("zone").cloned().unwrap_or_else(|| DEFAULT_ZONE.to_string());
    let image_id = params.get("image-id").cloned().unwrap_or_else(|| DEFAULT_IMAGE_ID.to_string());
    let instance_type = params.get("instance-type").cloned().unwrap_or_else(|| DEFAULT_INSTANCE_TYPE.to_string());
    let system_disk_type = params.get("system-disk-type").cloned().unwrap_or_else(|| DEFAULT_SYSTEM_DISK_TYPE.to_string());
    let system_disk_size = params.get("system-disk-size")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(DEFAULT_SYSTEM_DISK_SIZE);
    let data_disk_type = params.get("data-disk-type").cloned().unwrap_or_else(|| DEFAULT_DATA_DISK_TYPE.to_string());
    let data_disk_size = params.get("data-disk-size")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(DEFAULT_DATA_DISK_SIZE);
    let bandwidth = params.get("bandwidth")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(DEFAULT_BANDWIDTH);
    let instance_count = params.get("instance-count")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(DEFAULT_INSTANCE_COUNT);
    let period = params.get("period")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(DEFAULT_PERIOD);
    let password = params.get("password").cloned().unwrap_or_else(|| DEFAULT_PASSWORD.to_string());
    
    // 确定计费类型
    let charge_type_str = params.get("charge-type").cloned().unwrap_or_else(|| DEFAULT_CHARGE_TYPE.to_string());
    let charge_type = match charge_type_str.as_str() {
        "prepaid" => InstanceChargeType::Prepaid,
        "spotpaid" => InstanceChargeType::Spotpaid,
        _ => InstanceChargeType::PostpaidByHour,
    };
    
    // 打印所有参数（包括默认值）
    println!("\n=== 查询参数信息 ===");
    println!("区域(Region): {}", region);
    println!("可用区(Zone): {}", zone);
    println!("镜像ID(ImageId): {}", image_id);
    println!("实例类型(InstanceType): {}", instance_type);
    println!("系统盘类型(SystemDiskType): {}", system_disk_type);
    println!("系统盘大小(SystemDiskSize): {}GB", system_disk_size);
    println!("数据盘类型(DataDiskType): {}", data_disk_type);
    println!("数据盘大小(DataDiskSize): {}GB", data_disk_size);
    println!("公网带宽(Bandwidth): {}Mbps", bandwidth);
    println!("实例数量(InstanceCount): {}", instance_count);
    println!("计费类型(ChargeType): {}", charge_type_str);
    if matches!(charge_type, InstanceChargeType::Prepaid) {
        println!("购买周期(Period): {}个月", period);
    }
    println!("实例密码: {}", password);
    println!("");
    
    println!("正在查询实例价格...");
    
    // 创建询价请求参数
    let request = InquiryPriceRunInstancesRequest {
        Placement: Placement {
            Zone: Some(zone),
            ProjectId: None,
            HostIds: None,
            HostIps: None,
            DedicatedClusterId: None,
        },
        ImageId: image_id,
        InstanceType: instance_type,
        SystemDisk: Some(SystemDisk {
            DiskType: Some(system_disk_type),
            DiskSize: Some(system_disk_size),
            DiskId: None,
        }),
        DataDisks: Some(vec![
            DataDisk {
                DiskType: data_disk_type,
                DiskSize: data_disk_size,
                DiskId: None,
                DeleteWithInstance: Some(true),
            }
        ]),
        InternetAccessible: Some(InternetAccessible {
            InternetChargeType: "TRAFFIC_POSTPAID_BY_HOUR".to_string(),
            InternetMaxBandwidthOut: bandwidth,
            PublicIpAssigned: Some(true),
            BandwidthPackageId: None,
        }),
        InstanceChargeType: Some(charge_type.clone()),
        InstanceChargePrepaid: if let InstanceChargeType::Prepaid = charge_type {
            Some(InstanceChargePrepaid {
                Period: period,
                RenewFlag: Some("NOTIFY_AND_AUTO_RENEW".to_string()),
            })
        } else {
            None
        },
        InstanceCount: Some(instance_count),
        LoginSettings: Some(LoginSettings {
            Password: Some(password),
            KeyIds: None,
            KeepImageLogin: None,
        }),
        EnhancedService: Some(EnhancedService {
            SecurityService: Some(RunSecurityServiceEnabled {
                Enabled: true,
            }),
            MonitorService: Some(RunMonitorServiceEnabled {
                Enabled: true,
            }),
            AutomationService: None,
        }),
        VirtualPrivateCloud: None,
    };
    
    // 发送询价请求
    match instance_price_service.inquiry_price_run_instances(&request, &region).await {
        Ok(response) => {
            println!("\n查询到的实例价格:");
            display_price(&response.Response.Price);
        },
        Err(err) => {
            println!("询价失败: {}", err);
        }
    }
    
    Ok(())
}

// 解析命令行参数
fn parse_args(args: &[String]) -> HashMap<String, String> {
    let mut params = HashMap::new();
    let mut i = 1;
    
    while i < args.len() {
        let arg = &args[i];
        
        // 处理--开头的参数
        if arg.starts_with("--") {
            let key = arg[2..].to_string();
            
            // 检查是否还有下一个参数作为值
            if i + 1 < args.len() && !args[i + 1].starts_with("--") {
                params.insert(key, args[i + 1].clone());
                i += 2;
            } else {
                // 没有值的参数视为布尔标志
                params.insert(key, "true".to_string());
                i += 1;
            }
        } else {
            // 跳过不是以--开头的参数
            i += 1;
        }
    }
    
    params
}

// 打印价格信息
fn display_price(price: &tencent_cloud_sdk::services::cvm::instance_price::Price) {
    println!("=== 实例价格信息 ===");
    if let Some(unit_price) = &price.InstancePrice.UnitPrice {
        println!("单价: {}", unit_price);
    }
    
    if let Some(charge_unit) = &price.InstancePrice.ChargeUnit {
        println!("计费单位: {}", charge_unit);
    }
    
    if let Some(original_price) = &price.InstancePrice.OriginalPrice {
        println!("原价: {}", original_price);
    }
    
    if let Some(discount_price) = &price.InstancePrice.DiscountPrice {
        println!("折扣价: {}", discount_price);
    }
    
    if let Some(bandwidth_price) = &price.BandwidthPrice {
        println!("\n=== 带宽价格信息 ===");
        
        if let Some(unit_price) = &bandwidth_price.UnitPrice {
            println!("带宽单价: {}", unit_price);
        }
        
        if let Some(charge_unit) = &bandwidth_price.ChargeUnit {
            println!("带宽计费单位: {}", charge_unit);
        }
        
        if let Some(original_price) = &bandwidth_price.OriginalPrice {
            println!("带宽原价: {}", original_price);
        }
        
        if let Some(discount_price) = &bandwidth_price.DiscountPrice {
            println!("带宽折扣价: {}", discount_price);
        }
    }
}

// 打印使用方法
fn print_usage() {
    println!("用法: cargo run --example instance_price_inquiry -- [参数]");
    println!("\n可用参数:");
    println!("  --help                       显示此帮助信息");
    println!("  --region <大区>              指定大区 (默认: {})", DEFAULT_REGION);
    println!("  --zone <可用区>              指定可用区 (默认: {})", DEFAULT_ZONE);
    println!("  --image-id <镜像ID>          指定镜像ID (默认: {})", DEFAULT_IMAGE_ID);
    println!("  --instance-type <实例类型>   指定实例类型 (默认: {})", DEFAULT_INSTANCE_TYPE);
    println!("  --system-disk-type <类型>    系统盘类型 (默认: {})", DEFAULT_SYSTEM_DISK_TYPE);
    println!("  --system-disk-size <大小>    系统盘大小(GB) (默认: {})", DEFAULT_SYSTEM_DISK_SIZE);
    println!("  --data-disk-type <类型>      数据盘类型 (默认: {})", DEFAULT_DATA_DISK_TYPE);
    println!("  --data-disk-size <大小>      数据盘大小(GB) (默认: {})", DEFAULT_DATA_DISK_SIZE);
    println!("  --bandwidth <带宽>           公网出带宽(Mbps) (默认: {})", DEFAULT_BANDWIDTH);
    println!("  --charge-type <收费类型>     计费类型 prepaid/postpaid/spotpaid (默认: {})", DEFAULT_CHARGE_TYPE);
    println!("  --period <周期>              包年包月购买周期(月) (默认: {})", DEFAULT_PERIOD);
    println!("  --instance-count <数量>      实例数量 (默认: {})", DEFAULT_INSTANCE_COUNT);
    println!("  --password <密码>            实例密码 (默认: {})", DEFAULT_PASSWORD);
    println!("\n示例:");
    println!("  cargo run --example instance_price_inquiry -- ");
    println!("  cargo run --example instance_price_inquiry -- --region ap-beijing --zone ap-beijing-3");
    println!("  cargo run --example instance_price_inquiry -- --instance-type S5.LARGE8 --charge-type prepaid --period 3");
} 