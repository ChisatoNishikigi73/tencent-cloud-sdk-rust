use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance::{InstanceService, DescribeInstancesRequest, Filter}
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取密钥
    let secret_id = env::var("TENCENTCLOUD_SECRET_ID")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_ID");
    let secret_key = env::var("TENCENTCLOUD_SECRET_KEY")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_KEY");

    // 创建客户端
    let client = TencentCloudClient::new(secret_id, secret_key);
    
    // 创建实例服务
    let instance_service = InstanceService::new(&client);
    
    // 设置区域
    let region = "ap-guangzhou";
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 {
        match args[1].as_str() {
            // 按ID查询实例
            "id" => {
                if args.len() < 3 {
                    println!("缺少实例ID参数，用法: cargo run --example instance_describe id <实例ID>");
                    return Ok(());
                }
                
                let instance_ids = args[2..].to_vec();
                println!("正在查询以下实例ID: {:?}", instance_ids);
                
                let request = DescribeInstancesRequest {
                    InstanceIds: Some(instance_ids),
                    Filters: None,
                    Offset: None,
                    Limit: None,
                };
                
                let response = instance_service.describe_instances(&request, region).await?;
                display_instances(&response.Response);
            },
            
            // 按可用区查询实例
            "zone" => {
                if args.len() < 3 {
                    println!("缺少可用区参数，用法: cargo run --example instance_describe zone <可用区>");
                    return Ok(());
                }
                
                let zone = &args[2];
                println!("正在查询可用区 {} 的实例", zone);
                
                let mut filters = Vec::new();
                filters.push(Filter {
                    Name: "zone".to_string(),
                    Values: vec![zone.clone()],
                });
                
                let request = DescribeInstancesRequest {
                    InstanceIds: None,
                    Filters: Some(filters),
                    Offset: None,
                    Limit: Some(20),
                };
                
                let response = instance_service.describe_instances(&request, region).await?;
                display_instances(&response.Response);
            },
            
            // 按实例名称查询实例
            "name" => {
                if args.len() < 3 {
                    println!("缺少实例名称参数，用法: cargo run --example instance_describe name <实例名称>");
                    return Ok(());
                }
                
                let name = &args[2];
                println!("正在查询名称为 {} 的实例", name);
                
                let mut filters = Vec::new();
                filters.push(Filter {
                    Name: "instance-name".to_string(),
                    Values: vec![name.clone()],
                });
                
                let request = DescribeInstancesRequest {
                    InstanceIds: None,
                    Filters: Some(filters),
                    Offset: None,
                    Limit: Some(20),
                };
                
                let response = instance_service.describe_instances(&request, region).await?;
                display_instances(&response.Response);
            },
            
            // 按标签查询实例
            "tag" => {
                if args.len() < 4 {
                    println!("缺少标签参数，用法: cargo run --example instance_describe tag <标签键> <标签值>");
                    return Ok(());
                }
                
                let tag_key = &args[2];
                let tag_value = &args[3];
                println!("正在查询标签键值对为 {}:{} 的实例", tag_key, tag_value);
                
                let mut filters = Vec::new();
                filters.push(Filter {
                    Name: format!("tag:{}", tag_key),
                    Values: vec![tag_value.clone()],
                });
                
                let request = DescribeInstancesRequest {
                    InstanceIds: None,
                    Filters: Some(filters),
                    Offset: None,
                    Limit: Some(20),
                };
                
                let response = instance_service.describe_instances(&request, region).await?;
                display_instances(&response.Response);
            },
            
            // 查询所有实例
            "all" => {
                println!("正在查询所有实例");
                
                let request = DescribeInstancesRequest {
                    InstanceIds: None,
                    Filters: None,
                    Offset: None,
                    Limit: Some(20),
                };
                
                let response = instance_service.describe_instances(&request, region).await?;
                display_instances(&response.Response);
            },
            
            // 按状态查询实例
            "state" => {
                if args.len() < 3 {
                    println!("缺少实例状态参数，用法: cargo run --example instance_describe state <状态>");
                    println!("支持的状态: PENDING, RUNNING, STOPPED, STOPPING, REBOOTING, STARTING, SHUTDOWN, TERMINATING");
                    return Ok(());
                }
                
                let state = &args[2];
                println!("正在查询状态为 {} 的实例", state);
                
                let mut filters = Vec::new();
                filters.push(Filter {
                    Name: "instance-state".to_string(),
                    Values: vec![state.clone()],
                });
                
                let request = DescribeInstancesRequest {
                    InstanceIds: None,
                    Filters: Some(filters),
                    Offset: None,
                    Limit: Some(20),
                };
                
                let response = instance_service.describe_instances(&request, region).await?;
                display_instances(&response.Response);
            },
            
            _ => {
                print_usage();
            }
        }
    } else {
        print_usage();
    }
    
    Ok(())
}

// 格式化显示实例信息
fn display_instances(response: &tencent_cloud_sdk::services::cvm::instance::DescribeInstancesResponse) {
    println!("找到 {} 个符合条件的实例:", response.TotalCount);
    println!("{:<20} {:<15} {:<15} {:<15} {:<15}", "实例ID", "实例名称", "状态", "实例类型", "可用区");
    println!("{}", "-".repeat(80));
    
    for instance in &response.InstanceSet {
        let instance_name = instance.InstanceName.as_ref().map_or("N/A", |s| s.as_str());
        println!("{:<20} {:<15} {:<15} {:<15} {:<15}", 
            instance.InstanceId, 
            instance_name, 
            instance.InstanceState, 
            instance.InstanceType, 
            instance.Placement.Zone
        );
    }
    
    if response.TotalCount > 0 && !response.InstanceSet.is_empty() {
        println!("\n详细信息示例 (第一个实例):");
        let instance = &response.InstanceSet[0];
        println!("ID: {}", instance.InstanceId);
        
        let instance_name = instance.InstanceName.as_ref().map_or("N/A", |s| s.as_str());
        println!("名称: {}", instance_name);
        
        println!("状态: {}", instance.InstanceState);
        println!("创建时间: {}", instance.CreatedTime);
        
        let os_name = instance.OsName.as_ref().map_or("N/A", |s| s.as_str());
        println!("操作系统: {}", os_name);
        
        println!("CPU: {} 核", instance.CPU);
        println!("内存: {} GB", instance.Memory);
        println!("内网IP: {:?}", instance.PrivateIpAddresses);
        if let Some(ips) = &instance.PublicIpAddresses {
            if !ips.is_empty() {
                println!("公网IP: {:?}", ips);
            }
        }
        
        let latest_operation = instance.LatestOperation.as_ref().map_or("N/A", |s| s.as_str());
        let latest_operation_state = instance.LatestOperationState.as_ref().map_or("N/A", |s| s.as_str());
        println!("最新操作: {}", latest_operation);
        println!("最新操作状态: {}", latest_operation_state);
        
        if let Some(tags) = &instance.Tags {
            if !tags.is_empty() {
                println!("标签:");
                for tag in tags {
                    println!("  - {}:{}", tag.Key, tag.Value);
                }
            }
        }
        
        // 显示VPC信息（如果有）
        if let Some(vpc) = &instance.VirtualPrivateCloud {
            println!("VPC ID: {}", vpc.VpcId);
            println!("子网ID: {}", vpc.SubnetId);
        }
        
        // 显示UUID（如果有）
        if let Some(uuid) = &instance.Uuid {
            println!("UUID: {}", uuid);
        }
    }
}

// 打印使用说明
fn print_usage() {
    println!("用法:");
    println!("  cargo run --example instance_describe [选项] [参数...]");
    println!();
    println!("选项:");
    println!("  id <实例ID...>               按实例ID查询");
    println!("  zone <可用区>                按可用区查询");
    println!("  name <实例名称>              按实例名称查询");
    println!("  tag <标签键> <标签值>        按标签查询");
    println!("  state <状态>                 按实例状态查询");
    println!("  all                          查询所有实例");
    println!();
    println!("示例:");
    println!("  cargo run --example instance_describe id ins-xxxxxxxx");
    println!("  cargo run --example instance_describe zone ap-guangzhou-4");
    println!("  cargo run --example instance_describe name my-instance");
    println!("  cargo run --example instance_describe tag project test");
    println!("  cargo run --example instance_describe state RUNNING");
    println!("  cargo run --example instance_describe all");
} 