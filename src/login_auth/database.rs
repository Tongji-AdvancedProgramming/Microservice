use core::panic;
use std::env;

use mysql_async::{Pool, Conn, Opts, OptsBuilder};
use once_cell::sync::OnceCell;
use tracing::{instrument, info};

const MYSQL_CONNSTR:&'static str = "mysql://hwapp_forum@10.80.42.245:3306/homework";
static DB_POOL:OnceCell<Pool> = OnceCell::new();

#[instrument]
pub fn init_mysql_pool(){
    let passwd: String;
    match env::var("HWDB_PASS"){
        Ok(value) => passwd = value,
        Err(_) => panic!("你没有设置HWDB_PASS环境变量")
    }

    let opts = Opts::from_url(&MYSQL_CONNSTR).unwrap();
    let opts = OptsBuilder::from_opts(opts)
        .pass(Some(passwd));
    let opts = Opts::from(opts);

    info!("初始化数据库线程池……");
    DB_POOL.set(mysql_async::Pool::new(opts))
        .unwrap_or_else(|_| {info!("尝试插入线程池时出错")});
    info!("初始化数据库线程池成功");
}

#[instrument]
pub async fn get_connect() -> Conn {
    info!("从链接池获取链接……");
    let conn = DB_POOL.get().expect("获取链接池失败").get_conn().await.expect("获取链接失败");
    info!("从链接池获取链接成功"); 
    conn
}