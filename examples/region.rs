use tencent_cloud_sdk::{TencentCloudClient, services::region::RegionService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取密钥
    let secret_id = std::env::var("TENCENTCLOUD_SECRET_ID")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_ID");
    let secret_key = std::env::var("TENCENTCLOUD_SECRET_KEY")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_KEY");

    // 创建客户端
    let client = TencentCloudClient::new(secret_id, secret_key);
    
    // 创建地域服务
    let region_service = RegionService::new(&client);
    
    // 查询地域列表
    println!("正在查询地域列表...");
    let regions = region_service.describe_regions().await?;
    println!("找到 {} 个地域:", regions.Response.RegionSet.len());
    
    // 收集所有可用地区
    let available_regions: Vec<_> = regions.Response.RegionSet.iter()
        .filter(|r| r.RegionState == "AVAILABLE")
        .collect();
    
    println!("其中 {} 个地区可用:", available_regions.len());
    
    // 遍历每个可用地区，查询其可用区
    for region in available_regions {
        println!("\n┌─ {} ({})", region.RegionName, region.Region);
        
        // 查询该地区的可用区
        match region_service.describe_zones(&region.Region).await {
            Ok(zones) => {
                if zones.Response.ZoneSet.is_empty() {
                    println!("└── 没有可用区");
                } else {
                    let zone_count = zones.Response.ZoneSet.len();
                    for (i, zone) in zones.Response.ZoneSet.iter().enumerate() {
                        let is_last = i == zone_count - 1;
                        let prefix = if is_last { "└── " } else { "├── " };
                        println!("{}[{}] {} - 状态: {}", 
                             prefix, zone.Zone, zone.ZoneName, zone.ZoneState);
                    }
                }
            },
            Err(err) => {
                println!("└── 查询可用区失败: {}", err);
            }
        }
    }
    
    Ok(())
} 