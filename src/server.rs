mod storage;
mod proto {
    tonic::include_proto!("kv");
}

use proto::kv_server::{Kv, KvServer};
use proto::{
    DelRequest, DelResponse, GetRequest, GetResponse, MaxRequest, MaxResponse, MinRequest,
    MinResponse, PutRequest, PutResponse, SumRequest, SumResponse,
};
use std::sync::Arc;
use storage::InMemory;
use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct Service {
    store: Arc<RwLock<InMemory>>,
}

impl Service {
    fn new() -> Self {
        Service {
            store: Arc::new(RwLock::new(InMemory::default())),
        }
    }
}

#[tonic::async_trait]
impl Kv for Service {
    async fn get(&self, req: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let req = req.into_inner();
        println!("{:?}", req);
        let value = self.store.read().await.get(req.key).cloned();
        Ok(Response::new(GetResponse { value }))
    }

    async fn put(&self, req: Request<PutRequest>) -> Result<Response<PutResponse>, Status> {
        let req = req.into_inner();
        println!("{:?}", req);

        let value = self.store.write().await.put(req.key, req.value);
        Ok(Response::new(PutResponse { value }))
    }

    async fn del(&self, req: Request<DelRequest>) -> Result<Response<DelResponse>, Status> {
        let req = req.into_inner();
        println!("{:?}", req);
        let value = self.store.write().await.del(req.key);
        Ok(Response::new(DelResponse { value }))
    }

    async fn min(&self, req: Request<MinRequest>) -> Result<Response<MinResponse>, Status> {
        let req = req.into_inner();
        println!("{:?}", req);
        let value = self.store.read().await.min().cloned();
        Ok(Response::new(MinResponse { value }))
    }

    async fn max(&self, req: Request<MaxRequest>) -> Result<Response<MaxResponse>, Status> {
        let req = req.into_inner();
        println!("{:?}", req);
        let value = self.store.read().await.max().cloned();
        Ok(Response::new(MaxResponse { value }))
    }

    async fn sum(&self, req: Request<SumRequest>) -> Result<Response<SumResponse>, Status> {
        let req = req.into_inner();
        println!("{:?}", req);
        let value = self.store.read().await.sum();
        Ok(Response::new(SumResponse { value }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let service = Service::new();

    println!("KV Service listening on {}", addr);

    Server::builder()
        .add_service(KvServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
