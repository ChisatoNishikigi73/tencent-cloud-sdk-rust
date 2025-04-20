use tencent_cloud_sdk::{
    TencentCloudClient,
    services::cvm::instance_operation::{InstanceOperationService, TerminateInstancesRequest}
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
    
    // 获取要退还的实例ID，可以从命令行参数传入
    let instance_ids: Vec<String> = env::args()
        .skip(1)  // 跳过程序名称
        .collect();
    
    if instance_ids.is_empty() {
        eprintln!("请提供至少一个实例ID作为命令行参数");
        eprintln!("用法: cargo run --example instance_terminate <实例ID1> [<实例ID2> ...]");
        return Ok(());
    }
    
    // 确认是否继续
    println!("将退还以下实例:");
    for id in &instance_ids {
        println!("- {}", id);
    }
    println!("警告: 这个操作不可逆！按量计费实例将被直接销毁，包年包月实例将被移至回收站。");
    println!("是否继续？[y/N]");
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() != "y" {
        println!("操作已取消");
        return Ok(());
    }
    
    // 创建退还实例请求
    let request = TerminateInstancesRequest {
        InstanceIds: instance_ids,
        ReleasePrepaidDataDisks: Some(false),  // 默认不释放包年包月数据盘
    };
    
    // 发送退还请求
    println!("正在退还实例...");
    match instance_operation_service.terminate_instances(&request, region).await {
        Ok(_) => {
            println!("实例退还请求已提交成功");
            println!("按量计费实例将被直接销毁，包年包月实例将被移至回收站");
        },
        Err(err) => {
            println!("退还实例失败: {}", err);
        }
    }
    
    Ok(())
} 