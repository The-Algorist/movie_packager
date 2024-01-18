use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{ByteStream, Client};
use tokio;

#[tokio::main]
async fn main() {
    let bucket = "my-bucket"; 
    let key = "path/to/object.txt";
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let resp = client.get_object()
                 .bucket(bucket)
                 .key(key)
                 .send()
                 .await
                 .unwrap();

    println!("Got object {}: ", key);

    let body = resp.body.unwrap();

    let mut stream = ByteStream::new(body);
    while let Some(result) = stream.try_next().await {
        let bytes = result.unwrap();
        print!("{}", String::from_utf8_lossy(&bytes)); 
    }

    println!();
}