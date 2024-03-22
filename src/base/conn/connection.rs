use crate::base::conn::router::Router;
use anyhow::{anyhow, Result};
use hyper::server::*;
use hyper_util::rt::TokioIo;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tower::service_fn;

pub struct Conn {
    io: TokioIo<TcpStream>,
}

impl Conn {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            io: TokioIo::new(stream),
        }
    }

    pub async fn start_conn(conn: Conn) -> Result<()> {
        let router = Router::default();
        if let Err(err) = conn::http1::Builder::new()
            .serve_connection(conn.io, router)
            .await
        {
            return Err(anyhow!("serve error!"));
        };
        Ok(())
    }
}

pub struct ConnPool {
    conn_num: Arc<RwLock<u32>>,
    // conns: Vec<Arc<Mutex<Conn>>>,
    runtime: Runtime,
}

impl ConnPool {
    pub fn new() -> Result<ConnPool> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(12)
            .enable_all()
            .build()?;
        Ok(ConnPool {
            conn_num: Arc::new(RwLock::new(0)),
            runtime,
        })
    }
    pub fn handle_conn(&mut self, conn: Conn) {
        self.runtime.spawn(async move {
            Conn::start_conn(conn).await.unwrap();
        });
        self.conn_num.write().unwrap().add(1);
    }
}
