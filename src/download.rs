use aws_sdk_s3::{Client, Error};
use aws_config::Region;
use std::fs::File;
use std::io::Write;

async fn get_object_from_s3(bucket_name: &str, object_key: &str) -> Result<(), Error> {
    let region = Region::new("your-region"); // Replace with your AWS region
    let config = aws_config::from_env().region(region).load().await;
    let client = Client::new(&config);

    let get_object_req = client.get_object().bucket(bucket_name).key(object_key);
    let resp = get_object_req.send().await?;

    let body = resp.body.collect().await?;
    let mut file = File::create(object_key)?;
    file.write_all(&body)?;

    println!("Object downloaded successfully to {}", object_key);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let bucket_name = "your-bucket-name"; // Replace with your bucket name
    let object_key = "your-object-key"; // Replace with your object key

    get_object_from_s3(bucket_name, object_key).await?;

    Ok(())
}
