use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_: Value, _: Context) -> Result<Value, Error> {
    // let first_name = event["firstName"].as_str().unwrap_or("world");
    // Ok(json!({"statusCode": 200, "body": {"message": "hello from Rust!"}}))
    Ok(json!({
    "statusCode": 200,
    "body": "{\"message\": \"hello world from rust\"}",
    }))
}
