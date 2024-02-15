use redis::ConnectionLike;
use redis::JsonCommands;
use redis::Connection;
use serde_json::json;
fn main() {
    let con = connection_redis().ok().unwrap();
    let is_open = &con.is_open();
    println!("isOk: {}", is_open);
    let is_ok = set_json().is_ok();
    println!("set json is_ok: {}", is_ok);

}
 
fn connection_redis() -> redis::RedisResult<Connection> {
    let client = redis::Client::open("redis://test@127.0.0.1:6379/")?;
    let con = client.get_connection()?;
    //con.json_set("my_key", "$", &json!({"item": 42i32}).to_string())?;
    Ok(con)
}

fn set_json() -> redis::RedisResult<()> {
    let mut con = connection_redis().ok().unwrap();
    con.json_set("my_key", "$", &json!({"item": 42i32}).to_string())?;
    Ok(())
}

