use kv::{
    DelRequest, DelResponse, GetRequest, GetResponse, KvClient, MaxRequest, MaxResponse,
    MinRequest, MinResponse, PutRequest, PutResponse, SumRequest, SumResponse,
};
use rustyline::Editor;
use tonic::Request;

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("Missing key for `{0}`")]
    MissingKey(String),
    #[error("Missing value for `PUT`")]
    MissingValue,
    #[error("unknown command `{0}`")]
    UnknownCommand(String),
    #[error("unknown error")]
    UnknownError,
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ReadlineError(#[from] rustyline::error::ReadlineError),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

enum Command {
    Get(String),
    Put(String, i64),
    Del(String),
    Min,
    Max,
    Sum,
}

fn parse_input(input: &str) -> Result<Command, ParseError> {
    let mut input = input.trim().split(' ');
    let command = input.next().map(|c| c.to_uppercase());
    match command.as_deref() {
        Some("GET") => {
            let key = input
                .next()
                .map(|v| v.trim())
                .ok_or_else(|| ParseError::MissingKey("GET".into()))?;
            Ok(Command::Get(key.to_string()))
        }
        Some("PUT") => {
            let key = input
                .next()
                .map(|v| v.trim())
                .ok_or_else(|| ParseError::MissingKey("PUT".into()))?;
            let value = input
                .next()
                .map(|v| v.trim())
                .ok_or_else(|| ParseError::MissingValue)?;
            let value = value.parse::<i64>()?;
            Ok(Command::Put(key.to_string(), value))
        }
        Some("DEL") => {
            let key = input
                .next()
                .map(|v| v.trim())
                .ok_or_else(|| ParseError::MissingKey("DEL".into()))?;
            Ok(Command::Del(key.to_string()))
        }
        Some("MIN") => Ok(Command::Min),
        Some("MAX") => Ok(Command::Max),
        Some("SUM") => Ok(Command::Sum),
        Some(other) => Err(ParseError::UnknownCommand(other.into())),
        None => Err(ParseError::UnknownError),
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = KvClient::connect("http://[::1]:50051").await?;

    let mut reader = Editor::<()>::new();
    let _ = reader.load_history("history.txt");

    loop {
        let line = reader.readline("KV > ");

        match line {
            Ok(input) => {
                match parse_input(input.as_str()) {
                    Ok(Command::Get(key)) => {
                        let GetResponse { value } = client
                            .get(Request::new(GetRequest { key }))
                            .await?
                            .into_inner();
                        println!("{}", value.map(|v| v.to_string()).unwrap_or("null".into()));
                    }
                    Ok(Command::Put(key, value)) => {
                        let PutResponse { value } = client
                            .put(Request::new(PutRequest { key, value }))
                            .await?
                            .into_inner();
                        println!("{}", value.map(|v| v.to_string()).unwrap_or("null".into()));
                    }
                    Ok(Command::Del(key)) => {
                        let DelResponse { value } = client
                            .del(Request::new(DelRequest { key }))
                            .await?
                            .into_inner();
                        println!("{}", value.map(|v| v.to_string()).unwrap_or("null".into()));
                    }
                    Ok(Command::Min) => {
                        let MinResponse { value } =
                            client.min(Request::new(MinRequest {})).await?.into_inner();
                        println!("{}", value.map(|v| v.to_string()).unwrap_or("null".into()));
                    }
                    Ok(Command::Max) => {
                        let MaxResponse { value } =
                            client.max(Request::new(MaxRequest {})).await?.into_inner();
                        println!("{}", value.map(|v| v.to_string()).unwrap_or("null".into()));
                    }
                    Ok(Command::Sum) => {
                        let SumResponse { value } =
                            client.sum(Request::new(SumRequest {})).await?.into_inner();
                        println!("{}", value);
                    }
                    Err(err) => eprintln!("{}", err),
                }

                reader.add_history_entry(input.as_str());
            }
            Err(_err) => {
                break;
            }
        }
    }

    reader.save_history("history.txt")?;

    Ok(())
}
