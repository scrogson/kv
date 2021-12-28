use kv::{DelRequest, GetRequest, KvClient, MaxRequest, MinRequest, PutRequest, SumRequest};
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvClient::connect("http://[::1]:50051").await?;

    let response = client
        .put(Request::new(PutRequest {
            key: "A".into(),
            value: 10,
        }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client
        .put(Request::new(PutRequest {
            key: "B".into(),
            value: 20,
        }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client
        .put(Request::new(PutRequest {
            key: "C".into(),
            value: 30,
        }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client
        .get(Request::new(GetRequest { key: "A".into() }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client
        .get(Request::new(GetRequest { key: "B".into() }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client
        .get(Request::new(GetRequest { key: "C".into() }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client.min(Request::new(MinRequest {})).await?;

    println!("{:?}", response.into_inner());

    let response = client.max(Request::new(MaxRequest {})).await?;

    println!("{:?}", response.into_inner());

    let response = client.sum(Request::new(SumRequest {})).await?;

    println!("{:?}", response.into_inner());

    let response = client
        .del(Request::new(DelRequest { key: "A".into() }))
        .await?;

    println!("{:?}", response.into_inner());

    let response = client.sum(Request::new(SumRequest {})).await?;

    println!("{:?}", response.into_inner());

    Ok(())
}
