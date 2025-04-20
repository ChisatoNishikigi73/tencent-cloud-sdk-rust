use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance_operation::{
        InstanceOperationService, StartInstancesRequest, StopInstancesRequest, RebootInstancesRequest
    }
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
    
    // 创建实例操作服务
    let instance_operation_service = InstanceOperationService::new(&client);
    
    // 设置区域
    let region = "ap-guangzhou";
    
    // 获取要操作的实例ID
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("用法: cargo run --example instance_lifecycle <操作> <实例ID1> [<实例ID2> ...]");
        eprintln!("操作可以是: start, stop, reboot");
        return Ok(());
    }
    
    let operation = &args[1];
    let instance_ids: Vec<String> = args[2..].iter().cloned().collect();
    
    // 确认是否继续
    println!("将对以下实例执行{}操作:", operation);
    for id in &instance_ids {
        println!("- {}", id);
    }
    println!("是否继续？[y/N]");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() != "y" {
        println!("操作已取消");
        return Ok(());
    }
    
    match operation.as_str() {
        "start" => {
            // 创建启动实例请求
            let request = StartInstancesRequest {
                InstanceIds: instance_ids,
            };
            
            // 发送启动请求
            println!("正在启动实例...");
            match instance_operation_service.start_instances(&request, region).await {
                Ok(_) => {
                    println!("实例启动请求已提交成功");
                    println!("实例将从STOPPED状态变为STARTING状态，然后再变为RUNNING状态");
                },
                Err(err) => {
                    println!("启动实例失败: {}", err);
                }
            }
        },
        "stop" => {
            // 创建关闭实例请求
            let request = StopInstancesRequest {
                InstanceIds: instance_ids,
                StopType: Some("SOFT".to_string()),  // 软关机
                ForceStop: None,  // 弃用的参数
                StoppedMode: Some("KEEP_CHARGING".to_string()),  // 关机继续收费
            };
            
            // 发送关闭请求
            println!("正在关闭实例...");
            match instance_operation_service.stop_instances(&request, region).await {
                Ok(_) => {
                    println!("实例关闭请求已提交成功");
                    println!("实例将从RUNNING状态变为STOPPING状态，然后再变为STOPPED状态");
                },
                Err(err) => {
                    println!("关闭实例失败: {}", err);
                }
            }
        },
        "reboot" => {
            // 创建重启实例请求
            let request = RebootInstancesRequest {
                InstanceIds: instance_ids,
                StopType: Some("SOFT".to_string()),  // 软重启
                ForceReboot: None,  // 弃用的参数
            };
            
            // 发送重启请求
            println!("正在重启实例...");
            match instance_operation_service.reboot_instances(&request, region).await {
                Ok(_) => {
                    println!("实例重启请求已提交成功");
                    println!("实例将从RUNNING状态变为REBOOTING状态，然后再变为RUNNING状态");
                },
                Err(err) => {
                    println!("重启实例失败: {}", err);
                }
            }
        },
        _ => {
            eprintln!("未知操作: {}", operation);
            eprintln!("支持的操作: start, stop, reboot");
        }
    }
    
    Ok(())
} 