use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance_config::{
        InstanceConfigService, DescribeZoneInstanceConfigInfosRequest, Filter
    }
};
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

// 默认参数值
const DEFAULT_REGION: &str = "ap-guangzhou";
const DEFAULT_ZONE: &str = "ap-guangzhou-6";
const DEFAULT_CHARGE_TYPE: &str = "POSTPAID_BY_HOUR"; // 按量计费

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取密钥
    let secret_id = env::var("TENCENTCLOUD_SECRET_ID")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_ID");
    let secret_key = env::var("TENCENTCLOUD_SECRET_KEY")
        .expect("请设置环境变量TENCENTCLOUD_SECRET_KEY");

    // 创建客户端
    let client = TencentCloudClient::new(secret_id, secret_key);
    
    // 创建实例配置服务
    let instance_config_service = InstanceConfigService::new(&client);
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 解析命令行参数到HashMap
    let params = parse_args(&args);
    
    println!("参数: {:?}", params);
    
    // 如果指定了帮助参数，显示帮助信息并退出
    if params.contains_key("help") {
        print_usage();
        return Ok(());
    }
    
    // 从参数中获取值，如果没有则使用默认值
    let region = params.get("region").cloned().unwrap_or_else(|| DEFAULT_REGION.to_string());
    let zone = params.get("zone").cloned().unwrap_or_else(|| DEFAULT_ZONE.to_string());
    let charge_type = params.get("charge-type").cloned().unwrap_or_else(|| DEFAULT_CHARGE_TYPE.to_string());
    let instance_family = params.get("instance-family").cloned();
    let instance_type = params.get("instance-type").cloned();
    let sort_key = params.get("sort-key").cloned();
    let output_file = params.get("output").cloned();
    
    // 构建过滤条件
    let mut filters = Vec::new();
    
    // 添加可用区过滤条件
    filters.push(Filter {
        Name: "zone".to_string(),
        Values: vec![zone.clone()],
    });
    
    // 添加计费类型过滤条件
    filters.push(Filter {
        Name: "instance-charge-type".to_string(),
        Values: vec![charge_type.clone()],
    });
    
    // 如果指定了实例系列，则添加实例系列过滤条件
    if let Some(family) = instance_family {
        filters.push(Filter {
            Name: "instance-family".to_string(),
            Values: vec![family],
        });
    }
    
    // 如果指定了实例类型，则添加实例类型过滤条件
    if let Some(instance_type_value) = instance_type {
        filters.push(Filter {
            Name: "instance-type".to_string(),
            Values: vec![instance_type_value],
        });
    }
    
    // 如果指定了排序键，则添加排序键过滤条件
    if let Some(sort) = sort_key {
        filters.push(Filter {
            Name: "sort-keys".to_string(),
            Values: vec![sort],
        });
    }
    
    // 创建请求
    let request = DescribeZoneInstanceConfigInfosRequest {
        Filters: Some(filters),
    };
    
    println!("\n正在查询{}区域{}可用区的机型配置信息...", region, zone);
    
    // 发送请求获取可用区机型配置信息
    match instance_config_service.describe_zone_instance_config_infos(&request, &region).await {
        Ok(response) => {
            println!("\n===== 查询到的机型配置信息 =====");
            println!("共查询到{}个机型配置", response.Response.InstanceTypeQuotaSet.len());
            
            // 打印表头
            println!("{:<15} {:<15} {:<10} {:<10} {:<20} {:<20}", 
                "机型ID", "机型系列", "CPU(核)", "内存(GB)", "处理器类型", "状态");
            println!("{}", "-".repeat(95));
            
            // 准备输出文件内容（如果需要）
            let mut file_content = String::new();
            if output_file.is_some() {
                file_content.push_str("机型ID,机型系列,CPU(核),内存(GB),处理器类型,状态,网络带宽(Gbps),PPS(万),价格单位,原始价格,折扣价格\n");
            }
            
            // 打印每个机型的信息
            for item in response.Response.InstanceTypeQuotaSet {
                let cpu_type = item.CpuType.unwrap_or_else(|| "未知".to_string());
                println!("{:<15} {:<15} {:<10} {:<10.1} {:<20} {:<20}", 
                    item.InstanceType, 
                    item.InstanceFamily, 
                    item.Cpu, 
                    item.Memory, 
                    cpu_type, 
                    item.Status);
                
                // 如果需要输出到文件，添加到文件内容
                if let Some(_) = output_file.as_ref() {
                    let bandwidth = item.InstanceBandwidth.unwrap_or(0.0);
                    let pps = item.InstancePps.unwrap_or(0.0);
                    
                    let (charge_unit, original_price, discount_price) = if let Some(price) = item.Price {
                        (
                            price.ChargeUnit.unwrap_or_else(|| "".to_string()),
                            price.OriginalPrice.unwrap_or(0.0),
                            price.DiscountPrice.unwrap_or(0.0)
                        )
                    } else {
                        ("".to_string(), 0.0, 0.0)
                    };
                    
                    file_content.push_str(&format!("{},{},{},{:.1},{},{},{},{},{},{},{}\n",
                        item.InstanceType,
                        item.InstanceFamily,
                        item.Cpu,
                        item.Memory,
                        cpu_type,
                        item.Status,
                        bandwidth,
                        pps,
                        charge_unit,
                        original_price,
                        discount_price
                    ));
                }
            }
            
            // 如果指定了输出文件，则写入文件
            if let Some(filename) = output_file {
                match File::create(&filename) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(file_content.as_bytes()) {
                            println!("写入文件失败: {}", e);
                        } else {
                            println!("\n结果已保存到文件: {}", filename);
                        }
                    },
                    Err(e) => println!("创建文件失败: {}", e)
                }
            }
        },
        Err(err) => {
            println!("查询失败: {}", err);
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

// 打印使用方法
fn print_usage() {
    println!("用法: cargo run --example instance_config_info -- [参数]");
    println!("\n可用参数:");
    println!("  --help                       显示此帮助信息");
    println!("  --region <大区>              指定大区 (默认: {})", DEFAULT_REGION);
    println!("  --zone <可用区>              指定可用区 (默认: {})", DEFAULT_ZONE);
    println!("  --charge-type <计费类型>     计费类型，如PREPAID或POSTPAID_BY_HOUR (默认: {})", DEFAULT_CHARGE_TYPE);
    println!("  --instance-family <系列>     按照实例机型系列过滤，如S1、S2、S3等");
    println!("  --instance-type <类型>       按照实例机型过滤，如S3.MEDIUM4");
    println!("  --sort-key <排序键>          排序规则，如'cpu:desc'或'mem:asc'");
    println!("  --output <文件名>            将结果保存到CSV文件");
    println!("\n示例:");
    println!("  cargo run --example instance_config_info");
    println!("  cargo run --example instance_config_info -- --zone ap-guangzhou-3");
    println!("  cargo run --example instance_config_info -- --instance-family S5 --sort-key cpu:desc");
    println!("  cargo run --example instance_config_info -- --output instance_types.csv");
} 