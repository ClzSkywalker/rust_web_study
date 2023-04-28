#[tokio::main]
async fn main() {
    common::log::log::init_log("./logs");
    let _r = match orm::init_connection("./event_shop.db").await {
        Ok(r) => r,
        Err(r) => {
            println!("db err:{}",r);
            panic!()
        },
    };
    println!("Hello, world!");
}
