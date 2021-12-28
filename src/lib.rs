mod proto;
mod storage;

pub use storage::InMemory;

pub use proto::{
    DelRequest, DelResponse, GetRequest, GetResponse, Kv, KvClient, KvServer, MaxRequest,
    MaxResponse, MinRequest, MinResponse, PutRequest, PutResponse, SumRequest, SumResponse,
};
