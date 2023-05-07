use bfj_api;
use dotenvy;

fn main() {
    // 读取环境变量
    dotenvy::dotenv().ok();
    // 运行api服务
    bfj_api::start().expect("启动失败啦");
}
