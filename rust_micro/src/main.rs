use serde::{Deserialize};
use serde_json::{json, Value};
use simple_logger::SimpleLogger;
use aws_config::load_from_env;
use aws_sdk_dynamodb::{Client, model::AttributeValue};
use lambda_runtime::{LambdaEvent, Error as LambdaError, service_fn};

// write a function to post new information to AWS DynamoDB

#[derive(Deserialize)]
struct Request {
    id: Option<String>,
    name: Option<String>,
    age: Option<String>,
    gender: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    SimpleLogger::new().with_utc_timestamps().init()?;
    let func = service_fn(handler);
    lambda_runtime::run(func).await?;
    Ok(())
}


async fn handler(event: LambdaEvent<Value>) -> Result<Value, LambdaError> {
    let request: Request = serde_json::from_value(event.payload)?;

    let config = load_from_env().await;
    let client = Client::new(&config);

    post_new_info(&client, request.id, request.name, request.age, request.gender).await?;
    Ok(json!({"name":"Data Added Successfully"}))
    //Ok(json!({ "name": request.name, "address": request.address, "nationality": request.nationality, "status": "success" }))
}

async fn post_new_info(client: &Client, id: Option<String>, name: Option<String>, age: Option<String>, gender: Option<String>) -> Result<(), LambdaError> {
    let table_name = "IDS-721"; // Make sure this matches your DynamoDB table name
    let id_av = AttributeValue::S(id.expect("REASON"));
    let name_av = AttributeValue::S(name.expect("REASON"));
    let age_av = AttributeValue::S(age.expect("REASON"));
    let gender_av = AttributeValue::S(gender.expect("REASON"));

    client.put_item()
        .table_name(table_name)
        .item("id", id_av)
        .item("name", name_av)
        .item("age", age_av)
        .item("gender", gender_av)
        .send()
        .await?;

    Ok(())
}
